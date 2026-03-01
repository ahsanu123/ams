use chrono::NaiveDateTime;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::to_active_model_trait::ToActiveModel;

#[derive(Debug, Default, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BillingRetrieveDataWithBillAndAmount {
    pub billing_retrieve_data_id: i64,
    pub billing_id: i64,
    pub retrieve_data_id: i64,
    #[ts(type = "Date")]
    pub from: NaiveDateTime,
    #[ts(type = "Date")]
    pub to: NaiveDateTime,
    pub bill: f64,
    pub amount: i64,
}

impl BillingRetrieveDataWithBillAndAmount {
    pub fn with_other_data(
        model: Self,
        from: NaiveDateTime,
        to: NaiveDateTime,
        bill: f64,
        amount: i64,
    ) -> Self {
        Self {
            billing_retrieve_data_id: model.retrieve_data_id,
            billing_id: model.billing_id,
            retrieve_data_id: model.retrieve_data_id,
            from,
            to,
            bill,
            amount,
        }
    }
}

impl ToActiveModel<ams_entity::billing_retrieve_data::ActiveModel>
    for BillingRetrieveDataWithBillAndAmount
{
    fn to_active_without_id(&self) -> ams_entity::billing_retrieve_data::ActiveModel {
        ams_entity::billing_retrieve_data::ActiveModel {
            billing_retrieve_data_id: NotSet,
            billing_id: Set(self.billing_id),
            retrieve_data_id: Set(self.retrieve_data_id),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::billing_retrieve_data::ActiveModel {
        ams_entity::billing_retrieve_data::ActiveModel {
            billing_retrieve_data_id: Set(self.billing_retrieve_data_id),
            billing_id: Set(self.billing_id),
            retrieve_data_id: Set(self.retrieve_data_id),
        }
    }
}
