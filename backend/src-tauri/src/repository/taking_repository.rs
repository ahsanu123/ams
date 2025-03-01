use crate::model::taking_report::TakingRecord;
use chrono::NaiveDate;
use diesel::Connection;

pub struct TakingRepository {}

impl TakingRepository {
    fn add_record(conn: &mut impl Connection, data: &TakingRecord) {
        todo!()
    }
    fn get_records(conn: &mut impl Connection) -> Vec<TakingRecord> {
        todo!()
    }
    fn get_taken_dregs_for_some_date(date: NaiveDate) -> Vec<TakingRecord> {
        todo!()
    }
}
