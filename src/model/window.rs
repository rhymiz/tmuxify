use serde::{Deserialize, Serialize};

use super::Pane;

/// Available tmux window layouts
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WindowLayout {
    Tiled,
    EvenHorizontal,
    EvenVertical,
    MainHorizontal,
    MainVertical,
}

impl WindowLayout {
    /// Get all available layouts as a list
    pub fn all() -> &'static [WindowLayout] {
        &[
            WindowLayout::Tiled,
            WindowLayout::EvenHorizontal,
            WindowLayout::EvenVertical,
            WindowLayout::MainHorizontal,
            WindowLayout::MainVertical,
        ]
    }
}

impl std::fmt::Display for WindowLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowLayout::Tiled => write!(f, "tiled"),
            WindowLayout::EvenHorizontal => write!(f, "even-horizontal"),
            WindowLayout::EvenVertical => write!(f, "even-vertical"),
            WindowLayout::MainHorizontal => write!(f, "main-horizontal"),
            WindowLayout::MainVertical => write!(f, "main-vertical"),
        }
    }
}

/// Represents a tmux window with optional name, layout, and panes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<WindowLayout>,
    pub panes: Vec<Pane>,
}

impl Window {
    /// Create a new window with the given configuration
    pub fn new(name: Option<String>, layout: Option<WindowLayout>, panes: Vec<Pane>) -> Self {
        Self {
            window_name: name,
            layout,
            panes,
        }
    }

    /// Create a simple window with a single empty pane
    #[allow(dead_code)]
    pub fn simple() -> Self {
        Self {
            window_name: None,
            layout: None,
            panes: vec![Pane::empty()],
        }
    }
}
