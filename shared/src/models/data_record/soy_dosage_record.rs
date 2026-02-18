use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SoyDosageCountRecord {
    pub date: NaiveDateTime,
    pub count: i64,
}
