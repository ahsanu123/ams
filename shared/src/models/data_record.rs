use ams_entity::data_record;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

mod produced_dreg_record;
mod production_record;
mod soy_dosage_record;

use produced_dreg_record::ProducedDregRecord;
use production_record::ProductionRecord;
use soy_dosage_record::SoyDosageCountRecord;

#[derive(Serialize, Deserialize)]
pub enum RetrieveDataType {
    Production,
    SoyDosageCount,
    ProducedDreg,
}

pub enum DataRecordValueType {
    Production(ProductionRecord),
    SoyDosageCount(SoyDosageCountRecord),
    ProducedDreg(ProducedDregRecord),
}

pub struct DataRecord<KEY> {
    pub data_record_id: i64,
    pub key: KEY,
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

impl From<data_record::Model> for DataRecord<RetrieveDataType> {
    fn from(value: data_record::Model) -> Self {
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
}
