use ams_lib::model::paid_status::PaidStatus;
use chrono::NaiveDate;
use diesel::Connection;

pub struct PaymentRepository {}

impl PaymentRepository {
    fn add_bill_record(conn: &mut impl Connection, bill: &PaidStatus) {
        todo!()
    }
    fn update_payment_status(
        conn: &mut impl Connection,
        status: &PaidStatus,
        from: &NaiveDate,
        to: &NaiveDate,
    ) {
        todo!()
    }
}
