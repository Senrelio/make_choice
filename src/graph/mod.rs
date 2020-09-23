use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use sqlx::PgPool;
use sqlx::types::Uuid;

mod requirement;

pub struct RawGraph {
    pub docs: Vec<RawDoc>,
    pub edges: Vec<RawEdge>,
}

impl RawGraph {
    async fn load_all(pool: &PgPool) -> Result<Self> {
        let docs: Vec<RawDoc> = sqlx::query_as::<_, RawDoc>("select * from public.docs where flag_active = true;")
            .fetch_all(pool).await.unwrap();
        let edges: Vec<RawEdge> = sqlx::query_as::<_, RawEdge>("select * from public.edges where flag_active = true;")
            .fetch_all(pool).await.unwrap();
        Ok(RawGraph {
            docs: docs,
            edges: edges,
        })
    }
    async fn load_download(pool: &PgPool) -> Result<Self> {}
}

pub struct GraphWrapper {
    raw_graph: RawGraph,
}

impl GraphWrapper {
    async fn load_all(pool: &PgPool) -> Result<Self> {
        let raw_graph = RawGraph::load_all(pool).await.unwrap();
        Ok(GraphWrapper {
            raw_graph
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct RawDoc {
    pub id: Uuid,
    pub doc: Value,
}

impl RawDoc {
    pub fn is_tail_of<'a>(&self, graph: &'a GraphWrapper) -> Vec<&'a RawDoc> {
        graph.raw_graph.edges.iter().filter(|&e| e.tail_id == self.id)
            .map(|e| -> &RawDoc {
                graph.raw_graph.docs.iter().find(|&d| d.id == e.head_id).unwrap()
            }).collect()
    }
}

#[derive(sqlx::FromRow)]
pub struct RawEdge {
    pub id: Uuid,
    pub tail_id: Uuid,
    pub head_id: Uuid,
    pub label: String,
    pub properties: Value,
}

pub async fn new_doc<D>(poll: &PgPool, doc: &D) -> Result<Uuid> where D: Serialize {
    let query = format!("insert into public.docs (doc) values ('{}') returning id;",
                        serde_json::to_string(doc).unwrap());
    let uuid = sqlx::query_as::<_, (Uuid, )>(&query).fetch_one(poll).await.unwrap().0;
    Ok(uuid)
}

pub async fn get_docs(poll: &PgPool) -> Result<Vec<RawDoc>> {
    let docs = sqlx::query_as::<_, RawDoc>("select * from public.docs where flag_active = true;")
        .fetch_all(poll).await.unwrap_or_else(|_| vec![]);
    Ok(docs)
}

pub async fn get_doc_by_id(pool: &PgPool, id: &Uuid) -> Result<RawDoc> {
    let doc = sqlx::query_as::<_, RawDoc>(format!("select * from public.docs where id = '{}' and flag_active = true;", id.to_string()).as_str())
        .fetch_one(pool).await.unwrap();
    Ok(doc)
}

pub async fn new_edge(pool: &PgPool, edge: (Uuid, Uuid, String, &Value)) -> Result<Uuid> {
    let query = format!("insert into public.edges (tail_id, head_id, label, properties) values ('{}','{}','{}','{}') returning id;",
                        edge.0, edge.1, edge.2, edge.3.to_string());
    let uuid = sqlx::query_as::<_, (Uuid, )>(&query).fetch_one(pool).await.unwrap().0;
    Ok(uuid)
}

pub async fn get_edge_by_id(pool: &PgPool, id: Uuid) -> Result<RawEdge> {
    let query = format!("select * from public.edges where id = '{}' and flag_active = true;", id.to_string());
    let edge = sqlx::query_as::<_, RawEdge>(&query).fetch_one(pool).await.unwrap();
    Ok(edge)
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use sqlx::PgPool;
    use sqlx::types::Uuid;

    use crate::config;
    use crate::database;

    use super::*;

    #[derive(Serialize)]
    struct DummyDoc {
        pub name: String,
        pub age: u8,
        pub fetishes: Vec<String>,
    }

    impl DummyDoc {
        fn random() -> Self {
            use rand::Rng;
            use rand::thread_rng;
            use rand::distributions::Alphanumeric;
            use rand::seq::SliceRandom;
            use std::iter;
            let mut rng = thread_rng();
            let name: String = iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(8)
                .collect();
            let age: u8 = rng.gen_range(10, 90);
            let fetish_choice = vec!["leg".to_string(), "boob".to_string(), "inverted nipple".to_string(), "jk".to_string()];
            let fetishes: Vec<String> = fetish_choice.choose_multiple(&mut rng, 2).cloned().collect();
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
            .execute(poll).await.unwrap();
    }

    #[async_std::test]
    async fn new_doc_and_get_all() {
        let pool = get_pool().await;
        let doc = DummyDoc::random();
        let uuid: Uuid = new_doc(&pool, &doc).await.unwrap();
        let the_doc = get_doc_by_id(&pool, &uuid).await.unwrap();
        assert_eq!(the_doc.id, uuid);
        let doc_2 = DummyDoc::random();
        new_doc(&pool, &doc_2).await.unwrap();
        let docs = get_docs(&pool).await.unwrap();
        assert_eq!(docs.len(), 2);
        truncate_docs(&pool).await;
    }

    #[async_std::test]
    async fn new_edge_and_get_heads_for_tail() {
        let pool = get_pool().await;
        let id_1: Uuid = new_doc(&pool, &DummyDoc::random()).await.unwrap();
        let id_2: Uuid = new_doc(&pool, &DummyDoc::random()).await.unwrap();
        let _id_edge: Uuid = new_edge(&pool, (id_1, id_2, "test".to_string(), &serde_json::json!({
            "test": "test",
            "num": 10,
            "vec": ["song","wang","maipian"]
        }))).await.unwrap();
        let edge: RawEdge = get_edge_by_id(&pool, _id_edge).await.unwrap();
        assert_eq!(_id_edge, edge.id);
        let graph: GraphWrapper = GraphWrapper {
            raw_graph: RawGraph::load_all(&pool).await.unwrap()
        };
        let the_doc = graph.raw_graph.docs.iter().find(|&d| d.id == id_1).unwrap();
        let heads = the_doc.is_tail_of(&graph);
        assert_eq!(heads[0].id, id_2);
        truncate_docs(&pool).await;
    }
}
