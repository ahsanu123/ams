use crate::models::{
    balance::Balance, billing::Billing, customer::Customer,
    to_active_without_id_trait::ToActiveModel,
};
use ams_entity::balance::Model as BalanceModel;
use ams_entity::balance_billing::Model as BalanceBillingModel;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BalanceBilling {
    pub balance_billing_id: i64,
    pub balance_id: i64,
    pub billing_id: i64,

    pub balance: Balance,
    pub billing: Billing,
}

impl BalanceBilling {
    pub fn with_balance_and_billing(
        model: BalanceBillingModel,
        balance: Balance,
        billing: Billing,
    ) -> Self {
        Self {
            balance_billing_id: model.balance_billing_id,
            balance_id: model.balance_id,
            billing_id: model.billing_id,
            balance,
            billing,
        }
    }
    pub fn with_balance_model_and_billing(
        model: BalanceBillingModel,
        balance: BalanceModel,
        billing: Billing,
    ) -> Self {
        let balance = Balance::with_customer(balance, billing.clone().customer);

        Self {
            balance_billing_id: model.balance_billing_id,
            balance_id: model.balance_id,
            billing_id: model.billing_id,
            balance,
            billing,
        }
    }
}

impl ToActiveModel<ams_entity::balance_billing::ActiveModel> for BalanceBilling {
    fn to_active_without_id(&self) -> ams_entity::balance_billing::ActiveModel {
        ams_entity::balance_billing::ActiveModel {
            balance_billing_id: NotSet,
            balance_id: Set(self.balance_id),
            billing_id: Set(self.billing_id),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::balance_billing::ActiveModel {
        ams_entity::balance_billing::ActiveModel {
            balance_billing_id: Set(self.balance_billing_id),
            balance_id: Set(self.balance_id),
            billing_id: Set(self.billing_id),
        }
    }
}
