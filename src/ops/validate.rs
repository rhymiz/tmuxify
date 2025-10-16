use anyhow::{Result, anyhow};

/// Dependency that needs to be validated
#[derive(Debug)]
pub struct Dependency {
    pub name: &'static str,
    pub binary: &'static str,
    pub brew_package: &'static str,
}

impl Dependency {
    /// Check if the dependency is installed
    pub fn is_installed(&self) -> bool {
        which::which(self.binary).is_ok()
    }

    /// Get installation hint for missing dependency
    pub fn install_hint(&self) -> String {
        format!("brew install {}", self.brew_package)
    }
}

/// All required dependencies
pub const DEPENDENCIES: &[Dependency] = &[
    Dependency {
        name: "tmux",
        binary: "tmux",
        brew_package: "tmux",
    },
    Dependency {
        name: "tmuxp",
        binary: "tmuxp",
        brew_package: "tmuxp",
    },
    Dependency {
        name: "direnv",
        binary: "direnv",
        brew_package: "direnv",
    },
];

/// Validate that all required dependencies are installed
pub fn check_dependencies() -> Result<()> {
    let mut missing = Vec::new();

    for dep in DEPENDENCIES {
        if !dep.is_installed() {
            missing.push(dep);
        }
    }

    if !missing.is_empty() {
        let mut error_msg = String::from("Missing required dependencies:\n");
        for dep in &missing {
            error_msg.push_str(&format!(
                "  {} - install with: {}\n",
                dep.name,
                dep.install_hint()
            ));
        }
        return Err(anyhow!(error_msg));
    }

    Ok(())
}

/// Detect which shell is being used
pub fn detect_shell() -> Option<String> {
    std::env::var("SHELL")
        .ok()
        .and_then(|shell| shell.split('/').next_back().map(|s| s.to_string()))
}

/// Check if direnv hook is configured in shell RC file
pub fn check_direnv_hook() -> Result<bool> {
    let shell = detect_shell().unwrap_or_else(|| "zsh".to_string());

    let rc_file = match shell.as_str() {
        "zsh" => dirs::home_dir().map(|h| h.join(".zshrc")),
        "bash" => dirs::home_dir().map(|h| h.join(".bashrc")),
        _ => None,
    };

    if let Some(rc_path) = rc_file {
        if rc_path.exists() {
            let content = std::fs::read_to_string(&rc_path)?;
            let hook_pattern = format!("direnv hook {}", shell);
            return Ok(content.contains(&hook_pattern));
        }
    }

    Ok(false)
}

/// Get the direnv hook line for the current shell
pub fn get_direnv_hook_line() -> String {
    let shell = detect_shell().unwrap_or_else(|| "zsh".to_string());
    format!("eval \"$(direnv hook {})\"", shell)
}

/// Get the shell RC file path for the current shell
pub fn get_shell_rc_path() -> Option<String> {
    let shell = detect_shell().unwrap_or_else(|| "zsh".to_string());

    dirs::home_dir().map(|home| {
        let rc_file = match shell.as_str() {
            "zsh" => ".zshrc",
            "bash" => ".bashrc",
            _ => ".zshrc",
        };
        home.join(rc_file).display().to_string()
    })
}
