use chrono::NaiveDate;

pub struct Report {
    pub id: u32,
    pub title: String,
    pub date: NaiveDate,
    pub description: String,
}
