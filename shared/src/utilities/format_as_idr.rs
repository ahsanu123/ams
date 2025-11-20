use num_format::{Locale, ToFormattedString};

pub fn format_as_idr(num: i64) -> String {
    num.to_formatted_string(&Locale::id)
}
