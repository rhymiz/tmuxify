use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
    pub fn get_file_path(
        &self,
        location: TmuxpLocation,
        project_dir: Option<&Path>,
    ) -> anyhow::Result<PathBuf> {
        match location {
            TmuxpLocation::Home => {
                let home = dirs::home_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
                let tmuxp_dir = home.join(".tmuxp");
                Ok(tmuxp_dir.join(format!("{}.yaml", self.session_name)))
            }
            TmuxpLocation::Project => {
                if let Some(dir) = project_dir {
                    Ok(dir.join(".tmuxp.yaml"))
                } else {
                    Ok(PathBuf::from(".tmuxp.yaml"))
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn sample_config() -> Config {
        Config::new(
            "myapp".to_string(),
            "/tmp/myapp".to_string(),
            vec![],
        )
    }

    #[test]
    fn yaml_contains_basic_fields() {
        let cfg = sample_config();
        let y = cfg.to_yaml().unwrap();
        assert!(y.contains("session_name"));
        assert!(y.contains("myapp"));
        assert!(y.contains("start_directory"));
    }

    #[test]
    fn envrc_home_points_to_home_tmuxp() {
        let cfg = sample_config();
        let envrc = cfg.generate_envrc(TmuxpLocation::Home);
        assert!(envrc.contains("~/.tmuxp/myapp.yaml"));
    }

    #[test]
    fn envrc_project_points_to_local_file() {
        let cfg = sample_config();
        let envrc = cfg.generate_envrc(TmuxpLocation::Project);
        assert!(envrc.contains("./.tmuxp.yaml"));
    }

    #[test]
    fn get_path_home_includes_session() {
        let cfg = sample_config();
        let p = cfg.get_file_path(TmuxpLocation::Home, None).unwrap();
        assert!(p.ends_with(PathBuf::from(".tmuxp").join("myapp.yaml")));
    }

    #[test]
    fn get_path_project_uses_given_dir() {
        let cfg = sample_config();
        let p = cfg
            .get_file_path(TmuxpLocation::Project, Some(Path::new("/work/proj")))
            .unwrap();
        assert_eq!(p, PathBuf::from("/work/proj/.tmuxp.yaml"));
    }
}
