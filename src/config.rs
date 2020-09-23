use std::env;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Setting {
    debug: bool,
    database_url: String,
}

impl Setting {
    pub fn debug(&self) -> bool {
        self.debug
    }
    pub fn database_url(&self) -> &str {
        self.database_url.as_str()
    }
    pub fn init() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        match env.as_str() {
            "dev" => { s.set("debug", false).expect("set mode error"); }
            "test" => { s.set("debug", false).expect("set mode error"); }
            _ => ()
        };
        s.merge(File::with_name(&format!("configs/{}", env)).required(true))?;
        let setting = Setting {
            debug: env.as_str() == "dev",
            database_url: s.get("database.url").expect("database -> url doesn't exist."),
        };
        Ok(setting)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Setting;

    #[async_std::test]
    async fn check_database_url() {
        let setting = Setting::init().unwrap();
        assert_eq!(setting.database_url, "postgres://postgres:postgres@localhost:5432/nodes".to_owned());
    }
}