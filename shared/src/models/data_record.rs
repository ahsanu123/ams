use ams_entity::data_record;
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

mod produced_dreg_record;
mod production_record;
mod soy_dosage_record;

use produced_dreg_record::ProducedDregRecord;
use production_record::ProductionRecord;
use soy_dosage_record::SoyDosageCountRecord;
use ts_rs::TS;

use crate::models::to_active_model_trait::ToActiveModel;

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[ts(type = "number")]
pub enum RetrieveDataType {
    Production,
    SoyDosageCount,
    ProducedDreg,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum DataRecordValueType {
    Production(ProductionRecord),
    SoyDosageCount(SoyDosageCountRecord),
    ProducedDreg(ProducedDregRecord),
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct DataRecord<KEY> {
    pub data_record_id: i64,
    pub key: KEY,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
    pub value: DataRecordValueType,
}

pub type DefaultDataRecordType = DataRecord<RetrieveDataType>;

impl From<i64> for RetrieveDataType {
    fn from(value: i64) -> Self {
        match value {
            0 => RetrieveDataType::Production,
            1 => RetrieveDataType::SoyDosageCount,
            2 => RetrieveDataType::ProducedDreg,
            val => panic!("cant find coresponding RetrieveDataType for {}", val),
        }
    }
}

impl From<RetrieveDataType> for i64 {
    fn from(value: RetrieveDataType) -> Self {
        match value {
            RetrieveDataType::Production => 0,
            RetrieveDataType::SoyDosageCount => 1,
            RetrieveDataType::ProducedDreg => 2,
        }
    }
}

fn from_model_to_retrieve_data_type(value: data_record::Model) -> DataRecord<RetrieveDataType> {
    let key_ty: RetrieveDataType = value.key.into();

    match key_ty {
        RetrieveDataType::Production => {
            let production_record: ProductionRecord =
                serde_json::from_str(&value.json_value).unwrap_or_default();

            DataRecord {
                data_record_id: value.data_record_id,
                key: key_ty,
                date: value.date,
                value: DataRecordValueType::Production(production_record),
            }
        }

        RetrieveDataType::SoyDosageCount => {
            let soydosage_count: SoyDosageCountRecord =
                serde_json::from_str(&value.json_value).unwrap_or_default();

            DataRecord {
                data_record_id: value.data_record_id,
                key: key_ty,
                date: value.date,
                value: DataRecordValueType::SoyDosageCount(soydosage_count),
            }
        }

        RetrieveDataType::ProducedDreg => {
            let produced_dreg: ProducedDregRecord =
                serde_json::from_str(&value.json_value).unwrap_or_default();

            DataRecord {
                data_record_id: value.data_record_id,
                key: key_ty,
                date: value.date,
                value: DataRecordValueType::ProducedDreg(produced_dreg),
            }
        }
    }
}

impl From<&data_record::Model> for DataRecord<RetrieveDataType> {
    fn from(value: &data_record::Model) -> Self {
        from_model_to_retrieve_data_type(value.clone())
    }
}

impl From<data_record::Model> for DataRecord<RetrieveDataType> {
    fn from(value: data_record::Model) -> Self {
        from_model_to_retrieve_data_type(value)
    }
}

impl ToActiveModel<ams_entity::data_record::ActiveModel> for DefaultDataRecordType {
    fn to_active_without_id(&self) -> ams_entity::data_record::ActiveModel {
        let json_value = serde_json::to_string(&self.value).unwrap_or_default();

        ams_entity::data_record::ActiveModel {
            data_record_id: NotSet,
            key: Set(self.key.clone().into()),
            date: Set(self.date),
            json_value: Set(json_value),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::data_record::ActiveModel {
        let json_value = serde_json::to_string(&self.value).unwrap_or_default();

        ams_entity::data_record::ActiveModel {
            data_record_id: Set(self.data_record_id),
            key: Set(self.key.clone().into()),
            date: Set(self.date),
            json_value: Set(json_value),
        }
    }
}
