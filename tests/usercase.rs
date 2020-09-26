extern crate node_travel;

use serde::Serialize;
use sqlx::PgPool;

use node_travel::config;
use node_travel::database;

#[derive(Serialize)]
struct DummyDoc {
    pub name: String,
    pub age: u8,
    pub fetishes: Vec<String>,
}

impl DummyDoc {
    fn random() -> Self {
        use rand::distributions::Alphanumeric;
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        use rand::Rng;
        use std::iter;
        let mut rng = thread_rng();
        let name: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(8)
            .collect();
        let age: u8 = rng.gen_range(10, 90);
        let fetish_choice = vec![
            "leg".to_string(),
            "boob".to_string(),
            "inverted nipple".to_string(),
            "jk".to_string(),
        ];
        let fetishes: Vec<String> = fetish_choice
            .choose_multiple(&mut rng, 2)
            .cloned()
            .collect();
        DummyDoc {
            name,
            age,
            fetishes,
        }
    }
}

async fn get_pool() -> PgPool {
    let setting = config::Setting::init().unwrap();
    database::init_pool(setting.database_url()).await.unwrap()
}

async fn truncate_docs(poll: &PgPool) {
    sqlx::query("truncate table public.docs;")
        .execute(poll)
        .await
        .unwrap();
}

#[async_std::test]
async fn simple() {}

#[test]
fn json_test() {
    let json = serde_json::json!({
        "type": "requirement 1"
    });
    let t: &str = json["type"].as_str().unwrap();
    println!("{}", t);
}
