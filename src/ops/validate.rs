use anyhow::{Result, anyhow};

/// Dependency that needs to be validated
#[derive(Debug)]
pub struct Dependency {
    pub name: &'static str,
    pub binary: &'static str,
    pub package_name: &'static str,
}

impl Dependency {
    /// Check if the dependency is installed
    pub fn is_installed(&self) -> bool {
        which::which(self.binary).is_ok()
    }

    /// Get installation hint for missing dependency, adapted to available package manager
    pub fn install_hint(&self) -> String {
        // Probe common package managers
        let managers = [
            ("brew", format!("brew install {}", self.package_name)),
            (
                "apt-get",
                format!(
                    "sudo apt-get update && sudo apt-get install -y {}",
                    self.package_name
                ),
            ),
            (
                "apt",
                format!(
                    "sudo apt update && sudo apt install -y {}",
                    self.package_name
                ),
            ),
            ("dnf", format!("sudo dnf install -y {}", self.package_name)),
            ("pacman", format!("sudo pacman -S --noconfirm {}", self.package_name)),
            ("zypper", format!("sudo zypper install -y {}", self.package_name)),
        ];

        for (bin, cmd) in managers {
            if which::which(bin).is_ok() {
                return cmd;
            }
        }

        // Fallback generic hint
        format!("Install '{}' using your system's package manager", self.package_name)
    }
}

/// All required dependencies
pub const DEPENDENCIES: &[Dependency] = &[
    Dependency {
        name: "tmux",
        binary: "tmux",
        package_name: "tmux",
    },
    Dependency {
        name: "tmuxp",
        binary: "tmuxp",
        package_name: "tmuxp",
    },
    Dependency {
        name: "direnv",
        binary: "direnv",
        package_name: "direnv",
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

/// Check if currently running inside a tmux session
pub fn is_inside_tmux() -> bool {
    std::env::var("TMUX").is_ok()
}

/// Get the current tmux session name if inside tmux
pub fn get_current_tmux_session() -> Option<String> {
    if !is_inside_tmux() {
        return None;
    }

    // Try to get session name from tmux
    std::process::Command::new("tmux")
        .args(["display-message", "-p", "#S"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}
