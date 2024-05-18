use std::env;
use std::path::PathBuf;

pub enum ConfigKey {
    BaseDir,
}

pub struct AppConfig {
    pub base_dir: PathBuf,
}

impl AppConfig {
    pub fn load() -> Self {
        let base_dir = Self::resolve_env(ConfigKey::BaseDir);
        AppConfig { base_dir }
    }

    fn resolve_env(key: ConfigKey) -> PathBuf {
        match key {
            ConfigKey::BaseDir => env::var("HBOX_DIR")
                .unwrap_or_else(|_| "~/.hbox".to_string())
                .into(),
        }
    }

    pub fn config_file_path(&self) -> PathBuf {
        self.base_dir.join("config.json")
    }

    pub fn index_path(&self) -> PathBuf {
        self.base_dir.join("index")
    }

    pub fn overrides_path(&self) -> PathBuf {
        self.base_dir.join("overrides")
    }

    pub fn versions_path(&self) -> PathBuf {
        self.base_dir.join("versions")
    }

    pub fn shims_path(&self) -> PathBuf {
        self.base_dir.join("shims")
    }

    pub fn logs_path(&self) -> PathBuf {
        self.base_dir.join("logs")
    }
}
