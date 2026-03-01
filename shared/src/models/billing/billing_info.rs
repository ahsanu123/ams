use crate::{
    models::{
        customer::Customer, price::Price,
        retrieve_data::retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
    },
    sqls::billing::{get_billing_info_by_date, get_billing_info_by_date_and_customer_id},
};
use chrono::NaiveDateTime;
use itertools::{self, Itertools};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BillingInfo {
    #[ts(type = "Date")]
    pub from: NaiveDateTime,
    #[ts(type = "Date")]
    pub to: NaiveDateTime,

    pub retrieve_data: Vec<RetrieveDataWithCustomerAndPrice>,

    pub bill: f64,
    pub amount: i64,
}

impl From<get_billing_info_by_date::GetQueryResult>
    for (BillingInfo, Vec<RetrieveDataWithCustomerAndPrice>)
{
    fn from(value: get_billing_info_by_date::GetQueryResult) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test_billing_info_models {
    use itertools::Itertools;

    #[derive(Debug)]
    struct KeyVal {
        key: i64,
        val: String,
    }

    #[test]
    fn test_chunk_by() {
        let key_vals = [
            KeyVal {
                key: 1,
                val: "satu".into(),
            },
            KeyVal {
                key: 1,
                val: "one".into(),
            },
            KeyVal {
                key: 2,
                val: "dua".into(),
            },
            KeyVal {
                key: 3,
                val: "tiga".into(),
            },
            KeyVal {
                key: 4,
                val: "four".into(),
            },
        ];

        for (key, val) in &key_vals.into_iter().chunk_by(|key_val| key_val.key) {
            println!("key => {}, vals: {:?}", key, val.collect::<Vec<KeyVal>>());
        }
    }
}
