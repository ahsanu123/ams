use crate::{
    models::data_record::DefaultDataRecordType,
    repositories::{DATA_RECORD_REPO, base_repository_trait::BaseRepository},
};
use chrono::{Month, NaiveDateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct DataRecordGetAllProps {
    year: Option<i32>,
    #[ts(type = "Date", optional)]
    date: Option<NaiveDateTime>,
    #[ts(type = "number", optional)]
    start_month: Option<Month>,
    #[ts(type = "number", optional)]
    end_month: Option<Month>,
}

pub trait DataRecordControllerTrait {
    async fn get_all(&mut self, props: DataRecordGetAllProps) -> Vec<DefaultDataRecordType>;
    async fn create(&mut self, data: DefaultDataRecordType) -> i64;
}

pub struct DataRecordController;

impl DataRecordControllerTrait for DataRecordController {
    async fn get_all(&mut self, props: DataRecordGetAllProps) -> Vec<DefaultDataRecordType> {
        match props {
            DataRecordGetAllProps {
                year: Some(year),
                date: None,
                start_month: None,
                end_month: None,
            } => DATA_RECORD_REPO
                .lock()
                .await
                .get_by_year(year)
                .await
                .unwrap_or_default(),

            DataRecordGetAllProps {
                year: None,
                date: Some(date),
                start_month: None,
                end_month: None,
            } => DATA_RECORD_REPO
                .lock()
                .await
                .get_by_date(date)
                .await
                .unwrap_or_default(),

            DataRecordGetAllProps {
                year: Some(year),
                date: None,
                start_month: Some(from),
                end_month: Some(to),
            } => DATA_RECORD_REPO
                .lock()
                .await
                .get_by_month(year, from, to)
                .await
                .unwrap_or_default(),

            _ => todo!(),
        }
    }

    async fn create(&mut self, data: DefaultDataRecordType) -> i64 {
        DATA_RECORD_REPO
            .lock()
            .await
            .create(data)
            .await
            .unwrap_or_default()
    }
}
