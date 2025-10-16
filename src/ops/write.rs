use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::model::{Config, TmuxpLocation};

/// Options for writing files
pub struct WriteOptions {
    pub dry_run: bool,
    pub force: bool,
}

/// Result of a write operation
pub struct WriteResult {
    pub tmuxp_path: PathBuf,
    pub envrc_path: PathBuf,
    pub tmuxp_backed_up: bool,
    pub envrc_backed_up: bool,
}

impl WriteResult {
    pub fn print_summary(&self) {
        println!("\nFiles generated:");

        if self.tmuxp_backed_up {
            println!("  {} (backed up existing file)", self.tmuxp_path.display());
        } else {
            println!("  {}", self.tmuxp_path.display());
        }

        if self.envrc_backed_up {
            println!("  {} (backed up existing file)", self.envrc_path.display());
        } else {
            println!("  {}", self.envrc_path.display());
        }
    }
}

/// Create a backup of a file if it exists
fn backup_file(path: &Path, force: bool) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    if force {
        // Force mode: no backup, just overwrite
        return Ok(false);
    }

    // Create backup with timestamp, preserving original filename
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = {
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid path for backup"))?;
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        parent.join(format!("{}.backup.{}", file_name, timestamp))
    };

    fs::copy(path, &backup_path)
        .with_context(|| format!("Failed to create backup at {}", backup_path.display()))?;

    Ok(true)
}

/// Write configuration files to disk
pub fn write_config(
    config: &Config,
    location: TmuxpLocation,
    project_dir: &Path,
    options: &WriteOptions,
) -> Result<WriteResult> {
    // Get file paths (centralized through Config)
    let tmuxp_path = config.get_file_path(location, Some(project_dir))?;

    let envrc_path = project_dir.join(".envrc");

    // Generate content
    let tmuxp_content = config.to_yaml()?;
    let envrc_content = config.generate_envrc(location);

    if options.dry_run {
        // Dry run: just print what would be written
        println!("\n[DRY RUN] Would write to: {}", tmuxp_path.display());
        println!("---");
        println!("{}", tmuxp_content);
        println!("---");

        println!("\n[DRY RUN] Would write to: {}", envrc_path.display());
        println!("---");
        println!("{}", envrc_content);
        println!("---");

        return Ok(WriteResult {
            tmuxp_path,
            envrc_path,
            tmuxp_backed_up: false,
            envrc_backed_up: false,
        });
    }

    // Ensure parent directories exist
    if let Some(parent) = tmuxp_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    // Backup existing files if needed
    let tmuxp_backed_up = backup_file(&tmuxp_path, options.force)?;
    let envrc_backed_up = backup_file(&envrc_path, options.force)?;

    // Write tmuxp config
    fs::write(&tmuxp_path, tmuxp_content)
        .with_context(|| format!("Failed to write {}", tmuxp_path.display()))?;

    // Write .envrc
    fs::write(&envrc_path, envrc_content)
        .with_context(|| format!("Failed to write {}", envrc_path.display()))?;

    Ok(WriteResult {
        tmuxp_path,
        envrc_path,
        tmuxp_backed_up,
        envrc_backed_up,
    })
}

/// Run direnv allow in the project directory
pub fn run_direnv_allow(project_dir: &Path) -> Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::process::Command;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Running direnv allow...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let output = Command::new("direnv")
        .arg("allow")
        .current_dir(project_dir)
        .output()
        .context("Failed to execute direnv allow")?;

    pb.finish_with_message("direnv allow completed");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("direnv allow failed: {}", stderr);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Config, TmuxpLocation, Window};
    use tempfile::tempdir;

    #[test]
    fn write_config_creates_backups_when_files_exist() {
        let dir = tempdir().unwrap();
        let project_dir = dir.path();

        // Pre-create files to trigger backup
        let tmuxp_path = project_dir.join(".tmuxp.yaml");
        let envrc_path = project_dir.join(".envrc");
        fs::write(&tmuxp_path, "existing tmuxp").unwrap();
        fs::write(&envrc_path, "existing envrc").unwrap();

        let cfg = Config::new("sess".into(), project_dir.display().to_string(), vec![Window::simple()]);
        let opts = WriteOptions { dry_run: false, force: false };

        let res = write_config(&cfg, TmuxpLocation::Project, project_dir, &opts).unwrap();

        assert!(res.tmuxp_backed_up);
        assert!(res.envrc_backed_up);

        // Look for backup files
        let mut tmuxp_backup_found = false;
        let mut envrc_backup_found = false;
        for entry in fs::read_dir(project_dir).unwrap() {
            let entry = entry.unwrap();
            let name = entry.file_name();
            let s = name.to_string_lossy();
            if s.starts_with(".tmuxp.yaml.backup.") { tmuxp_backup_found = true; }
            if s.starts_with(".envrc.backup.") { envrc_backup_found = true; }
        }

        assert!(tmuxp_backup_found, "expected tmuxp backup file");
        assert!(envrc_backup_found, "expected envrc backup file");
    }
}
