use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use crate::division::{CleanFrequency, Division};

pub struct DivisionRepo {}

impl DivisionRepo {
    pub fn get_divisions() -> Vec<Division> {
        let wc_last_cleaned_at = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let wc = Division {
            name: String::from("wc1"),
            last_cleaned_at: Utc.from_utc_datetime(&wc_last_cleaned_at),
            frequency: CleanFrequency::EveryTwoWeeks,
        };

        let living_last_cleaned_at = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 15).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let living = Division {
            name: String::from("living"),
            last_cleaned_at: Utc.from_utc_datetime(&living_last_cleaned_at),
            frequency: CleanFrequency::EveryTwoWeeks,
        };

        vec![wc, living]
    }

    pub fn update(name: String) -> bool {
        println!("Update {}", name);
        true
    }
}
