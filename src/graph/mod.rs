use anyhow::Result;
use petgraph::Graph;
use serde::Serialize;
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

mod requirement;

pub async fn new_doc<D>(poll: &PgPool, doc: &D) -> Result<Uuid>
where
    D: Serialize,
{
    let query = format!(
        "insert into public.docs (doc) values ('{}') returning id;",
        serde_json::to_string(doc).unwrap()
    );
    let uuid = sqlx::query_as::<_, (Uuid,)>(&query)
        .fetch_one(poll)
        .await
        .unwrap()
        .0;
    Ok(uuid)
}

pub async fn get_docs(poll: &PgPool) -> Result<Vec<RawDoc>> {
    let docs = sqlx::query_as::<_, RawDoc>("select * from public.docs where flag_active = true;")
        .fetch_all(poll)
        .await
        .unwrap_or_else(|_| vec![]);
    Ok(docs)
}

pub async fn get_doc_by_id(pool: &PgPool, id: &Uuid) -> Result<RawDoc> {
    let doc = sqlx::query_as::<_, RawDoc>(
        format!(
            "select * from public.docs where id = '{}' and flag_active = true;",
            id.to_string()
        )
        .as_str(),
    )
    .fetch_one(pool)
    .await
    .unwrap();
    Ok(doc)
}

pub async fn new_edge(pool: &PgPool, edge: (Uuid, Uuid, String, &Value)) -> Result<Uuid> {
    let query = format!("insert into public.edges (tail_id, head_id, label, properties) values ('{}','{}','{}','{}') returning id;",
                        edge.0, edge.1, edge.2, edge.3.to_string());
    let uuid = sqlx::query_as::<_, (Uuid,)>(&query)
        .fetch_one(pool)
        .await
        .unwrap()
        .0;
    Ok(uuid)
}

pub async fn get_edge_by_id(pool: &PgPool, id: Uuid) -> Result<RawEdge> {
    let query = format!(
        "select * from public.edges where id = '{}' and flag_active = true;",
        id.to_string()
    );
    let edge = sqlx::query_as::<_, RawEdge>(&query)
        .fetch_one(pool)
        .await
        .unwrap();
    Ok(edge)
}

#[derive(sqlx::FromRow)]
pub struct RawDoc {
    pub id: Uuid,
    pub doc: Value,
}

impl RawDoc {
    fn fulfilled(&self, graph: &GraphFacade) -> Option<bool> {
        let doc_type: String = self.doc["type"].to_string();
        unimplemented!()
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

pub struct GraphFacade {
    graph: Graph<RawDoc, RawEdge>,
}

impl GraphFacade {
    async fn init(pool: &PgPool) -> Result<Self> {
        let docs: Vec<RawDoc> =
            sqlx::query_as::<_, RawDoc>("select * from public.docs where flag_active = true;")
                .fetch_all(pool)
                .await
                .unwrap();
        let edges: Vec<RawEdge> =
            sqlx::query_as::<_, RawEdge>("select * from public.edges where flag_active = true;")
                .fetch_all(pool)
                .await
                .unwrap();
        unimplemented!("init graph facade")
    }
}

pub struct EndpointPair {}

impl EndpointPair {
    fn source(&self) -> Option<RawEdge> {
        unimplemented!()
    }
    fn target(&self) -> Option<RawEdge> {
        unimplemented!()
    }
}

impl GraphFacade {
    fn nodes(&self) -> HashSet<RawDoc> {
        unimplemented!("get nodes")
    }
    fn edges(&self) -> HashSet<RawEdge> {
        unimplemented!()
    }
    fn predecessors(&self, node: &RawDoc) -> HashSet<RawDoc> {
        unimplemented!()
    }
    fn successors(&self, node: &RawDoc) -> HashSet<RawDoc> {
        unimplemented!()
    }
    fn fulfilled(&self, node: &RawDoc) -> Option<bool> {
        node.fulfilled(&self)
    }
}
