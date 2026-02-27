use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct ProductionRecord {
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
    pub count: i64,
}
