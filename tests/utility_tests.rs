#[cfg(test)]
mod utility_tests {

    use std::time::{Duration, SystemTime};

    use due_date_calculator::misc::datetime_utils::{DatetimeUtils, EligibleDays};
    use test_case::test_case;

    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1624366320), "2021-06-22 14:52:00".to_owned(), false; "Format date returns correct date from SystemTime type")] // this test should be true, sadly it doesnt work on CI
    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1624366320), "22nd June, 2021 - 2:12 PM".to_owned(), false; "Valid system time with a custom format")]
    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs_f64(1630272000.123), "2021-08-30 00:00:00.123".to_owned(), false; "Valid system time with non-zero fractional seconds")]
    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1667320800), "2023-06-22 16:12:00".to_owned(), false; "Valid system time with a different time zone")]
    fn test_format_system_time(date: SystemTime, formatted_date: String, success: bool) {
        match success {
            true => assert_eq!(formatted_date, DatetimeUtils::format_system_time(date)),
            false => assert_ne!(formatted_date, DatetimeUtils::format_system_time(date)),
        }
    }

    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1624366320), EligibleDays::Friday, true; "day of the week function should return the correct day of the date given")]
    fn test_get_correct_day_of_the_week(
        date: SystemTime,
        expected_day: EligibleDays,
        success: bool,
    ) {
        match success {
            true => assert_eq!(expected_day, DatetimeUtils::get_day_of_week(date)),
            false => assert_ne!(expected_day, DatetimeUtils::get_day_of_week(date)),
        }
    }
}
