use crate::model::taking_record::TakingRecord;
use diesel::Connection;

pub struct TakingRepository {}

impl TakingRepository {
    fn add_record(conn: &mut impl Connection, data: &TakingRecord) {
        todo!()
    }
    fn get_records(conn: &mut impl Connection) -> Vec<TakingRecord> {
        todo!()
    }
}
