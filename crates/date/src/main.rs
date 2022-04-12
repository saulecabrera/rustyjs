wit_bindgen_rust::export!("../../wit/date.wit");

use chrono::{DateTime, Utc};
use chrono_humanize::{HumanTime, Accuracy, Tense};

pub struct Date {}

impl date::Date for Date {
    fn fmt(date1: String, date2: String) -> String {
        let start_date = DateTime::<Utc>::from_utc(
            chrono::NaiveDate::parse_from_str(&date1, "%d/%m/%Y")
                .unwrap()
                .and_hms(0, 0, 0),
            Utc,
        );

        let end_date = DateTime::<Utc>::from_utc(
            chrono::NaiveDate::parse_from_str(&date2, "%d/%m/%Y")
                .unwrap()
                .and_hms(0, 0, 0),
            Utc,
        );

        let diff = end_date.signed_duration_since(start_date);

        let ht = HumanTime::from(diff);
        ht.to_text_en(Accuracy::Rough, Tense::Present)
    }
}

fn main() {
}
