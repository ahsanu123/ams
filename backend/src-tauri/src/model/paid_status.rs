use chrono::NaiveDate;

pub struct PaidStatus {
    pub id: i32,
    pub user_id: i32,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub paid: bool,
}
