//! Local persistence boundary.

use std::path::{Path, PathBuf};

use anyhow::Context;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Settings is a flat persisted preference schema."
)]
pub struct Settings {
    pub schema_version: u32,
    pub theme: String,
    pub language: String,
    pub sidebar_open: bool,
    pub high_contrast: bool,
    pub onboarded: bool,
    pub log_level: String,
    pub launch_at_login: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            schema_version: 1,
            theme: "dark".to_owned(),
            language: "en".to_owned(),
            sidebar_open: true,
            high_contrast: false,
            onboarded: false,
            log_level: "info".to_owned(),
            launch_at_login: false,
        }
    }
}

pub struct Paths {
    pub data: PathBuf,
    pub config: PathBuf,
    pub logs: PathBuf,
}

impl Paths {
    /// Resolves per-user application directories.
    ///
    /// # Errors
    ///
    /// Returns an error when the OS has no usable project directories.
    pub fn discover(app_name: &str) -> anyhow::Result<Self> {
        let dirs = ProjectDirs::from("dev", "smokeforge", app_name)
            .context("could not resolve application directories")?;
        let _ = std::fs::create_dir_all(dirs.data_dir());
        let _ = std::fs::create_dir_all(dirs.config_dir());
        let _ = std::fs::create_dir_all(dirs.cache_dir());
        Ok(Self {
            data: dirs.data_dir().to_path_buf(),
            config: dirs.config_dir().to_path_buf(),
            logs: dirs.cache_dir().to_path_buf(),
        })
    }
}

pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    #[must_use]
    pub fn new(config_dir: &Path) -> Self {
        Self {
            path: config_dir.join("settings.json"),
        }
    }

    /// Loads settings, quarantining corrupt or unknown-schema files.
    ///
    /// # Errors
    ///
    /// Returns an error when the file exists but cannot be read.
    pub fn load(&self) -> Result<Settings, String> {
        let raw = match std::fs::read_to_string(&self.path) {
            Ok(raw) => raw,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Settings::default());
            }
            Err(error) => return Err(error.to_string()),
        };

        match serde_json::from_str::<Settings>(&raw) {
            Ok(settings) if settings.schema_version == 1 => Ok(settings),
            Ok(_) | Err(_) => {
                let corrupt = self.path.with_extension("json.corrupt");
                let _ = std::fs::rename(&self.path, corrupt);
                Ok(Settings::default())
            }
        }
    }

    /// Saves settings atomically.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be written.
    pub fn save(&self, settings: &Settings) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let temporary = self.path.with_extension("json.tmp");
        let raw = serde_json::to_string_pretty(settings).map_err(|error| error.to_string())?;
        std::fs::write(&temporary, format!("{raw}\n")).map_err(|error| error.to_string())?;
        std::fs::rename(temporary, &self.path).map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("smokeforge-gpui-{name}-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn corrupt_settings_are_quarantined() {
        let dir = temp_dir("settings");
        let store = SettingsStore::new(&dir);
        std::fs::write(dir.join("settings.json"), "{broken").unwrap();
        let settings = store.load().unwrap();
        assert_eq!(settings.schema_version, 1);
        assert!(dir.join("settings.json.corrupt").exists());
    }
}
