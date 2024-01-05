use directories::BaseDirs;
use serde::Deserialize;

use std::fs;

#[derive(Deserialize, Default)]
pub struct ServerConfig {
    pub db: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl ServerConfig {
    pub fn set_db(&mut self, db: &Option<String>) {
        if db.is_some() {
            self.db = db.clone();
        }
    }
    pub fn set_host(&mut self, host: &Option<String>) {
        if host.is_some() {
            self.host = host.clone();
        }
    }
    pub fn set_port(&mut self, port: &Option<u16>) {
        if port.is_some() {
            self.port = *port;
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub host: Option<String>,
    pub db: Option<String>,
    pub server: Option<ServerConfig>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: None,
            db: None,
            server: Some(ServerConfig::default()),
        }
    }
}

impl Config {
    pub fn read() -> Option<Config> {
        let base_dirs = BaseDirs::new()?;
        let config_path = base_dirs
            .config_dir()
            .join("marcador")
            .join("marcador.toml");

        if config_path.exists() {
            toml::from_str::<Config>(&fs::read_to_string(config_path).ok()?).ok()
        } else {
            Some(Config::default())
        }
    }

    pub fn set_host(&mut self, host: &Option<String>) {
        if host.is_some() {
            self.host = host.clone();
        }
    }

    pub fn set_db(&mut self, db: &Option<String>) {
        if db.is_some() {
            self.db = db.clone();
        }
    }
}
