use serde::{Deserialize, Serialize};
use std::process::Command;
use tracing::{error, info};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AppHooks {
    pub before_tui: Option<String>,
    pub after_tui: Option<String>,
    pub open_link: Option<String>,
}

impl AppHooks {
    pub fn run_before_tui(&self) {
        if let Some(cmd) = &self.before_tui {
            Self::run_shell_command(cmd, "before_tui");
        }
    }

    pub fn run_after_tui(&self) {
        if let Some(cmd) = &self.after_tui {
            Self::run_shell_command(cmd, "after_tui");
        }
    }

    /// Returns the open_link command with %s substituted, or None if not configured.
    pub fn build_open_link_command(&self, url: &str) -> Option<String> {
        self.open_link.as_ref().map(|t| t.replace("%s", url))
    }

    fn run_shell_command(cmd: &str, hook_name: &str) {
        info!("Running hook '{}': {}", hook_name, cmd);
        match Command::new("sh").arg("-c").arg(cmd).status() {
            Ok(s) if s.success() => {}
            Ok(s) => error!("Hook '{}' exited with status: {}", hook_name, s),
            Err(e) => error!("Failed to execute hook '{}': {}", hook_name, e),
        }
    }
}
