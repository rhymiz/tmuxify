use anyhow::Result;
use console::style;
use dialoguer::{Confirm, Editor, Input, Select, theme::ColorfulTheme};

use crate::cli::Args;
use crate::model::{Config, Pane, TmuxpLocation, Window, WindowLayout};
use crate::ops::{validate, write};

/// Run the interactive configuration wizard
pub fn run(args: Args) -> Result<()> {
    println!("{}", style("Welcome to tmuxify!").bold().cyan());
    println!();

    // Check if running inside tmux
    if validate::is_inside_tmux() {
        eprintln!("{}", style("Warning:").yellow().bold());
        eprintln!("You are currently inside a tmux session.");

        if let Some(session_name) = validate::get_current_tmux_session() {
            eprintln!("Current session: {}", style(&session_name).cyan());
        }

        eprintln!();
        eprintln!("tmuxify is designed to create new tmux sessions.");
        eprintln!("Running it from within tmux may cause unexpected behavior.");
        eprintln!();

        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue anyway?")
            .default(false)
            .interact()?
        {
            println!("Aborted. Please run tmuxify from outside of tmux.");
            // Don't exit the process; return gracefully for testability
            return Ok(());
        }

        println!();
    }

    // Check dependencies first
    if let Err(e) = validate::check_dependencies() {
        eprintln!("{}", style("Error:").red().bold());
        eprintln!("{}", e);
        eprintln!();
        eprintln!(
            "Run {} to check your system configuration.",
            style("tmuxify doctor").yellow()
        );
        // Don't exit the process; return error to caller for testability
        return Err(e);
    }

    // Determine project directory
    let project_dir = args
        .project
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    // Determine session name
    let default_session_name = project_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-session")
        .to_string();

    let session_name = if let Some(name) = args.session {
        name
    } else {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Session name")
            .default(default_session_name)
            .interact_text()?
    };

    // Determine tmuxp location
    let location = if let Some(loc_str) = args.tmuxp_location {
        TmuxpLocation::from_str(&loc_str).ok_or_else(|| {
            anyhow::anyhow!("Invalid location: {}. Use 'home' or 'project'", loc_str)
        })?
    } else {
        let choices = vec!["home (~/.tmuxp/)", "project (./.tmuxp.yaml)"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Where should the tmuxp config be stored?")
            .items(&choices)
            .default(0)
            .interact()?;

        if selection == 0 {
            TmuxpLocation::Home
        } else {
            TmuxpLocation::Project
        }
    };

    // Determine start directory
    let start_dir = if let Some(dir) = args.start_dir {
        dir.display().to_string()
    } else {
        project_dir.display().to_string()
    };

    println!();
    println!("{}", style("Configuring windows and panes...").bold());
    println!();

    // Create windows interactively
    let mut windows = Vec::new();
    loop {
        let window = create_window_interactive(windows.len() + 1)?;
        windows.push(window);

        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add another window?")
            .default(false)
            .interact()?
        {
            break;
        }
    }

    // Create config
    let config = Config::new(session_name, start_dir, windows);

    // Show preview
    println!();
    println!("{}", style("Configuration preview:").bold().cyan());
    println!("---");
    println!("{}", config.to_yaml()?);
    println!("---");
    println!();

    // Confirm
    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with this configuration?")
        .default(true)
        .interact()?
    {
        println!("Aborted.");
        return Ok(());
    }

    // Write files
    let write_options = write::WriteOptions {
        dry_run: args.dry_run,
        force: args.force,
    };

    let result = write::write_config(&config, location, &project_dir, &write_options)?;

    if !args.dry_run {
        println!();
        result.print_summary();

        // Offer to run direnv allow
        println!();
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Run 'direnv allow' now?")
            .default(true)
            .interact()?
        {
            write::run_direnv_allow(&project_dir)?;
            println!();
            println!(
                "{}",
                style("✓ All done! Your tmux session is ready.")
                    .green()
                    .bold()
            );
            println!("  cd into this directory to automatically attach to your session.");
        }
    }

    Ok(())
}

/// Interactively create a window configuration
fn create_window_interactive(window_num: usize) -> Result<Window> {
    println!("{}", style(format!("Window #{}", window_num)).bold());

    // Window name
    let window_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("  Window name (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let window_name = if window_name.is_empty() {
        None
    } else {
        Some(window_name)
    };

    // Layout
    let layout_choices: Vec<String> = WindowLayout::all().iter().map(|l| l.to_string()).collect();

    let layout_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("  Layout")
        .items(&layout_choices)
        .default(0)
        .interact()?;

    let layout = Some(WindowLayout::all()[layout_idx]);

    // Number of panes
    let num_panes: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("  Number of panes")
        .default(1)
        .interact_text()?;

    // Create panes
    let mut panes = Vec::new();
    for pane_num in 1..=num_panes {
        let pane = create_pane_interactive(pane_num)?;
        panes.push(pane);
    }

    Ok(Window::new(window_name, layout, panes))
}

/// Interactively create a pane configuration
fn create_pane_interactive(pane_num: usize) -> Result<Pane> {
    println!("    {}", style(format!("Pane #{}", pane_num)).dim());

    let input_method = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("      Enter commands")
        .items(["Single line", "Multi-line (editor)", "No commands"])
        .default(0)
        .interact()?;

    let commands = match input_method {
        0 => {
            // Single line
            let cmd: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("      Command")
                .allow_empty(true)
                .interact_text()?;

            if cmd.is_empty() {
                Vec::new()
            } else {
                vec![cmd]
            }
        }
        1 => {
            // Multi-line editor
            if let Some(text) = Editor::new().edit("# Enter commands (one per line)\n")? {
                text.lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| line.to_string())
                    .collect()
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(), // No commands
    };

    Ok(Pane::new(commands))
}
