use sqlx::{Error, PgPool};
use sqlx::postgres::PgPoolOptions;

pub async fn init_pool(url: &str) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url).await
}

#[cfg(test)]
mod tests {
    use sqlx::types::Uuid;

    use crate::config;

    use super::init_pool;

    #[async_std::test]
    async fn poll_connection() {
        let setting = config::Setting::init().unwrap();
        let pool = init_pool(setting.database_url()).await.unwrap();
        let db_version = sqlx::query_as::<_, (String, )>("select version();").fetch_one(&pool).await.unwrap().0;
        assert!(db_version.to_lowercase().contains("postgres"));
        let uuid = sqlx::query_as::<_, (Uuid, )>("select uuid_generate_v4();").fetch_one(&pool).await.unwrap().0;
        assert_eq!(uuid.get_version_num(), 4);
    }
}