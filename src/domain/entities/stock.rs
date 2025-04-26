use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub last_updated: OffsetDateTime,
}

impl Stock {
    pub fn new(code: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            code,
            name,
            last_updated: OffsetDateTime::now_utc(),
        }
    }
}
