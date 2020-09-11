use serde::Deserialize;
use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub connection_size: u32,
}

impl Database {
    fn get_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", &self.username, &self.password, &self.host, &self.port, &self.database)
    }
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub database: Database
}

impl Setting {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        let env = std::env::var("RUN_MODE").unwrap_or_else(|_| String::from("dev"));
        s.merge(File::with_name(&format!("config/{}", env)).required(true))?;
        s.try_into()
    }
    pub fn get_database_url(&self) -> String {
        self.database.get_url()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{PgConnection, Connection};

    #[async_std::test]
    async fn read_setting_and_db_connection() {
        let setting = Setting::new().unwrap();
        PgConnection::connect(&setting.get_database_url()).await.unwrap();
    }
}