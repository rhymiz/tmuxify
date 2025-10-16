use serde::{Deserialize, Serialize};

/// Represents a single tmux pane with optional shell commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pane {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shell_command: Vec<String>,
}

impl Pane {
    /// Create a new pane with the given commands
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            shell_command: commands,
        }
    }

    /// Create an empty pane (no commands)
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            shell_command: Vec::new(),
        }
    }
}
