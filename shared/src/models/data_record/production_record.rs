use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ProductionRecord {
    pub date: NaiveDateTime,
    pub count: i64,
}
