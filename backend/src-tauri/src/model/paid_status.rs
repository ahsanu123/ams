use chrono::NaiveDate;

pub struct PaidStatus {
    pub username: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub paid: bool,
}
