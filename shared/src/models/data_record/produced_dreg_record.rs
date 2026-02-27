use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Default, Clone, TS)]
#[ts(export)]
pub struct ProducedDregRecord {
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
    pub count: i64,
}
