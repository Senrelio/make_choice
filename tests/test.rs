use sqlx::sqlite::SqlitePoolOptions;

use make_choice::database;

#[async_std::test]
async fn db_health_check() {
    let query_drop_table = "drop table if exists dev.dummy;";
    let query_new_table = "
        create table dev.dummy(
            id int8,
            name varchar
        );
    ";
    let query_new_entry = "insert into dev.dummy(id, name) values (1, 'song');";
    let query_select = "select * from dev.dummy limit 1;";
    let pool = database::create_polls().await;
    sqlx::query(query_drop_table).execute(&pool).await.unwrap();
    sqlx::query(query_new_table).execute(&pool).await.unwrap();
    sqlx::query(query_new_entry).execute(&pool).await.unwrap();
    let actual: (i64, String) = sqlx::query_as(query_select).fetch_one(&pool).await.unwrap();
    let expect: (i64, String) = (1, String::from("song"));
    assert_eq!(expect, actual);
    sqlx::query(query_drop_table).execute(&pool).await.unwrap();
}

#[async_std::test]
async fn sqlite_test() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./src/db/app.sqlite").await.unwrap();
}
