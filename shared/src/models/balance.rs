use crate::models::customer::Customer;
use chrono::NaiveDateTime;

// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub balance_id: i64,
//     pub customer_id: i64,
//     pub value: i64,
//     pub date: DateTime,
//     pub transaction_type: i64,
// }

pub enum TransactionType {
    TopUp,
    Pay,
}

impl From<i64> for TransactionType {
    fn from(value: i64) -> Self {
        match value {
            0 => TransactionType::TopUp,
            1 => TransactionType::Pay,
            val => panic!("cant find coresponding TransactionType for {}", val),
        }
    }
}

pub struct Balance {
    pub balance_id: i64,
    pub customer_id: i64,
    pub value: i64,
    pub date: NaiveDateTime,
    pub transaction_type: TransactionType,
    pub customer: Customer,
}
