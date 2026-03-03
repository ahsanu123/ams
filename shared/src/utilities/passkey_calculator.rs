use chrono::{Datelike, Local};

pub fn validate_passkey(passkey: i32) -> bool {
    let now = Local::now().naive_local();
    let mut date_month = (now.date().day() as i32) - (now.date().month() as i32);
    let month_year = (now.date().month() as i32) * now.date().year();

    if date_month == 0 {
        date_month = 1;
    } else if date_month < 0 {
        date_month = date_month.abs();
    }
    let calculated_pass_key = (date_month * month_year).abs();

    calculated_pass_key == passkey
}
