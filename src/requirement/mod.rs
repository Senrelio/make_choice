use serde_json::Value;
use serde::Deserialize;

pub mod user;
pub mod item;

pub static REQUIREMENT_TYPE: &str = "requirement";

pub enum Doc {
    Requirement(Requirement),
    Comment(Comment),
    Item(Item),
    Choice(Choice),
}


#[derive(sqlx::FromRow, Deserialize)]
pub struct Requirement {
    pub description: String,
    pub notes: Vec<String>,
}

impl From<Requirement> for Value {
    fn from(req: Requirement) -> Self {
        serde_json::json!({
            "type": REQUIREMENT_TYPE,
            "description": req.description,
            "notes": req.notes
        })
    }
}

pub struct Comment {}

pub struct Item {}

pub struct Choice {}