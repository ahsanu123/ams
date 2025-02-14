use chrono::NaiveDate;

pub trait ProductionRepositoryTrait {
    pub async fn add_production(amount: u32, date: NaiveDate);
    pub async fn get_production_error(date: NaiveDate);
}
