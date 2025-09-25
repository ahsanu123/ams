use chrono::{Datelike, NaiveDate, NaiveDateTime};

pub fn generate_start_end_month(date: NaiveDateTime) -> (NaiveDateTime, NaiveDateTime) {
    let start_month = date
        .date()
        .with_day(1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let end_month = if date.month() == 12 {
        NaiveDate::from_ymd_opt(date.year() + 1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    } else {
        NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    };

    (start_month, end_month)
}
