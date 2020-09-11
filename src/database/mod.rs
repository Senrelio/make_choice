use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::setting::Setting;

pub async fn create_polls() -> PgPool {
    let setting = Setting::new().expect("read config error");
    let database_url = setting.get_database_url();
    PgPoolOptions::new()
        .max_connections(setting.database.connection_size)
        .connect(&database_url).await.expect("create database pool error")
}