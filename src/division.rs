use chrono::{DateTime, Duration, Utc};

#[repr(i64)]
#[derive(Copy, Clone)]
enum CleanFrequency {
    EveryWeek = 7,
    EveryTwoWeeks = 14,
    OnceAMonth = 30,
}

struct Division {
    name: String,
    last_cleaned_at: DateTime<Utc>,
    frequency: CleanFrequency,
}

impl Division {
    fn dirtyness(&self, compare: Option<DateTime<Utc>>) -> f32 {
        let today = compare.unwrap_or(Utc::now());
        let frequency_days = self.frequency as i64;
        let next_clean_at = self.last_cleaned_at + Duration::days(frequency_days);
        let days_diff = (next_clean_at - today).num_days();
        let percentage = days_diff as f32 / frequency_days as f32;
        1.0 - percentage
    }

    fn clean(&mut self, cleaned_at: Option<DateTime<Utc>>) {
        self.last_cleaned_at = cleaned_at.unwrap_or(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

    use super::*;

    #[test]
    fn test_division_dirtyness_10_days_once_a_month() {
        let last_cleaned_at = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let bedroom = Division {
            name: String::from("Bedroom 1"),
            frequency: CleanFrequency::OnceAMonth,
            // 2024-07-01 16:00:00
            last_cleaned_at: Utc.from_utc_datetime(&last_cleaned_at),
        };

        // 2024-07-10 16:00:00
        let today = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 10).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let dirtyness = bedroom.dirtyness(Some(Utc.from_utc_datetime(&today)));

        assert_eq!(dirtyness, 0.3);
    }

    #[test]
    fn test_division_dirtyness_7_days_every_week() {
        let last_cleaned_at = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let bedroom = Division {
            name: String::from("Bedroom 1"),
            frequency: CleanFrequency::EveryWeek,
            // 2024-07-01 16:00:00
            last_cleaned_at: Utc.from_utc_datetime(&last_cleaned_at),
        };

        // 2024-07-08 16:00:00
        let today = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 8).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let dirtyness = bedroom.dirtyness(Some(Utc.from_utc_datetime(&today)));

        assert_eq!(dirtyness, 1.0);
    }

    #[test]
    fn test_division_clean_injected_today() {
        let last_cleaned_at = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let mut bedroom = Division {
            name: String::from("Bedroom 1"),
            frequency: CleanFrequency::EveryWeek,
            // 2024-07-01 16:00:00
            last_cleaned_at: Utc.from_utc_datetime(&last_cleaned_at),
        };

        // 2024-07-10 16:00:00
        let today = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 8).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        bedroom.clean(Some(Utc.from_utc_datetime(&today)));

        assert_eq!(bedroom.last_cleaned_at, Utc.from_utc_datetime(&today));
    }

    #[test]
    fn test_division_clean_today() {
        let last_cleaned_at = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let mut bedroom = Division {
            name: String::from("Bedroom 1"),
            frequency: CleanFrequency::EveryWeek,
            // 2024-07-01 16:00:00
            last_cleaned_at: Utc.from_utc_datetime(&last_cleaned_at),
        };

        bedroom.clean(None);

        assert_ne!(
            bedroom.last_cleaned_at,
            Utc.from_utc_datetime(&last_cleaned_at)
        );
    }
}
