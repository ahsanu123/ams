use chrono::NaiveDate;

pub trait ProductionRepositoryTrait {
    async fn add_production(amount: u32, date: NaiveDate);
    async fn get_production_error(date: NaiveDate);
}
