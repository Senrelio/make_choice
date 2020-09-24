use sqlx::types::Uuid;

use crate::graph::RawDoc;

pub struct Requirement {
    id: Uuid,
}

impl From<RawDoc> for Requirement {
    fn from(_: RawDoc) -> Self {
        unimplemented!()
    }
}
