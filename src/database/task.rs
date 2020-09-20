use serde_json::Value;

use crate::requirement::{Requirement, REQUIREMENT_TYPE};
use sqlx::types::Uuid;

pub enum DatabaseTask {
    NewRequirement(Requirement),
    GetRequirementById(Uuid),

}

pub struct QueryBuilder {}

impl QueryBuilder {
    fn build(task: DatabaseTask) -> String {
        match task {
            DatabaseTask::NewRequirement(req) => format!(
                "insert into dev.docs(doc) values (\'{}\') returning id;", Value::from(req).to_string()),
            DatabaseTask::GetRequirementById(id) => format!(
                "select * from dev.docs where id = '{}' and doc ->> 'type' = '{}';", id, REQUIREMENT_TYPE),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::database;
    use crate::database::task::DatabaseTask::{NewRequirement, GetRequirementById};
    use sqlx::types::Uuid;
    use sqlx::{Row, PgPool};

    async fn truncate_doc(pool: &PgPool) {
        sqlx::query!("truncate dev.docs;").execute(pool).await.unwrap();
    }

    #[async_std::test]
    async fn new_requirement() {
        let pool = database::create_polls().await;
        let req = Requirement {
            description: "first requirement".to_string(),
            notes: vec!["note1".to_owned(), "note2".to_owned()],
        };
        let query = QueryBuilder::build(NewRequirement(req));
        let row: (Uuid, ) = sqlx::query_as(&query).fetch_one(&pool).await.unwrap();
        let actual_id = row.0;
        assert_eq!(actual_id.get_version_num(), 4);
        truncate_doc(&pool).await;
    }

    #[async_std::test]
    async fn get_requirement_by_uuid() {
        let pool = database::create_polls().await;
        let req = Requirement {
            description: "first requirement".to_string(),
            notes: vec!["note1".to_owned(), "note2".to_owned()],
        };
        let insert_query = QueryBuilder::build(NewRequirement(req));
        let uuid = sqlx::query_as::<_, (Uuid, )>(&insert_query).fetch_one(&pool).await.unwrap().0;
        let select_query = QueryBuilder::build(GetRequirementById(uuid));
        let (id, json): (Uuid, Value, ) = sqlx::query_as(&select_query).fetch_one(&pool).await.unwrap();
        assert_eq!(id, uuid);
        assert_eq!(json.get("type").unwrap(), "requirement");
        let result: Requirement = serde_json::from_value(json).unwrap();
        assert_eq!(result.description, "first requirement");
        truncate_doc(&pool).await;
    }
}