use chrono::Utc;

use crate::division::{CleanFrequency, Division};

pub struct DivisionRepo {}

impl DivisionRepo {
    pub fn get_divisions() -> Vec<Division> {
        let wc = Division {
            name: String::from("wc1"),
            last_cleaned_at: Utc::now(),
            frequency: CleanFrequency::EveryTwoWeeks,
        };

        let living = Division {
            name: String::from("living"),
            last_cleaned_at: Utc::now(),
            frequency: CleanFrequency::EveryTwoWeeks,
        };

        vec![wc, living]
    }
}
