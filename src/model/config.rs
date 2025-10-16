use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::Window;

/// Where to store the tmuxp configuration file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmuxpLocation {
    /// Store in ~/.tmuxp/<session>.yaml
    Home,
    /// Store in project root as .tmuxp.yaml
    Project,
}

impl TmuxpLocation {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "home" => Some(TmuxpLocation::Home),
            "project" => Some(TmuxpLocation::Project),
            _ => None,
        }
    }
}

/// Complete tmuxp configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub session_name: String,
    pub start_directory: String,
    pub windows: Vec<Window>,
}

impl Config {
    /// Create a new tmuxp configuration
    pub fn new(session_name: String, start_directory: String, windows: Vec<Window>) -> Self {
        Self {
            session_name,
            start_directory,
            windows,
        }
    }

    /// Serialize to YAML string
    pub fn to_yaml(&self) -> anyhow::Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    /// Get the tmuxp file path based on location preference
    pub fn get_file_path(&self, location: TmuxpLocation) -> anyhow::Result<PathBuf> {
        match location {
            TmuxpLocation::Home => {
                let home = dirs::home_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
                let tmuxp_dir = home.join(".tmuxp");
                Ok(tmuxp_dir.join(format!("{}.yaml", self.session_name)))
            }
            TmuxpLocation::Project => Ok(PathBuf::from(".tmuxp.yaml")),
        }
    }

    /// Generate the .envrc content for this configuration
    pub fn generate_envrc(&self, location: TmuxpLocation) -> String {
        let load_path = match location {
            TmuxpLocation::Home => format!("~/.tmuxp/{}.yaml", self.session_name),
            TmuxpLocation::Project => "./.tmuxp.yaml".to_string(),
        };

        format!(
            r#"if [ -z "$TMUX" ]; then
  tmuxp load {}
fi
"#,
            load_path
        )
    }
}
