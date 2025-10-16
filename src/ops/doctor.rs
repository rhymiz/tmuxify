use anyhow::Result;
use console::style;

use crate::ops::validate;

/// Run diagnostics to check system dependencies and configuration
pub fn run() -> Result<()> {
    println!("{}", style("Running tmuxify doctor...").bold().cyan());
    println!();

    let mut all_ok = true;

    // Check each dependency
    println!("{}", style("Checking dependencies:").bold());
    for dep in validate::DEPENDENCIES {
        if dep.is_installed() {
            println!("  {} {}", style("✓").green().bold(), dep.name);
        } else {
            println!(
                "  {} {} - {}",
                style("✗").red().bold(),
                dep.name,
                style(format!("install with: {}", dep.install_hint())).dim()
            );
            all_ok = false;
        }
    }
    println!();

    // Check shell detection
    println!("{}", style("Shell configuration:").bold());
    match validate::detect_shell() {
        Some(shell) => {
            println!("  {} Detected shell: {}", style("✓").green().bold(), shell);

            // Check if direnv hook is configured
            match validate::check_direnv_hook() {
                Ok(true) => {
                    println!("  {} direnv hook configured", style("✓").green().bold());
                }
                Ok(false) => {
                    println!("  {} direnv hook not found", style("✗").red().bold());
                    if let Some(rc_path) = validate::get_shell_rc_path() {
                        println!("    Add this line to {}:", style(rc_path).cyan());
                        println!("    {}", style(validate::get_direnv_hook_line()).yellow());
                    }
                    all_ok = false;
                }
                Err(e) => {
                    println!(
                        "  {} Could not check direnv hook: {}",
                        style("⚠").yellow().bold(),
                        style(e).dim()
                    );
                }
            }
        }
        None => {
            println!("  {} Could not detect shell", style("⚠").yellow().bold());
        }
    }
    println!();

    // Final summary
    if all_ok {
        println!(
            "{}",
            style("✓ All checks passed! You're ready to use tmuxify.")
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            style("✗ Some issues found. Please address them before using tmuxify.")
                .red()
                .bold()
        );
    }

    Ok(())
}
