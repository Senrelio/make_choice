use sqlx::PgPool;

pub mod database;
pub mod setting;
pub mod requirement;
pub mod service;

pub async fn init_pool() -> PgPool {
    database::create_polls().await
}

pub async fn run_http_server(pool: &PgPool) {
    tide::log::start();
    let mut app = tide::new();
    app.at("/health_check").get(|_| async { Ok("hello world!") });
    app.listen("0.0.0.0:8080").await.unwrap();
}