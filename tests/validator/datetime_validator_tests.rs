#[cfg(test)]
mod syntax_validator_tests {
    use due_date_calculator::validator::localdatetime::DatetimeValidator;
    use std::time::{Duration, SystemTime};
    use test_case::test_case;

    #[test_case("2023-06-20T10:30:00Z", SystemTime::UNIX_EPOCH + Duration::from_secs(1687257000), true; "Validate Datetime should return successfully with the expected SystemTime")]
    #[test_case("2023-06-20T11:30:00Z", SystemTime::UNIX_EPOCH + Duration::from_secs(1687257000), false; "Validate Datetime should fail with the expected SystemTime")]
    fn test_syntax_localdatetime_validator(
        given_datetime: &str,
        expected_datetime: SystemTime,
        success: bool,
    ) {
        match success {
            true => assert_eq!(
                expected_datetime,
                DatetimeValidator::validate(given_datetime).unwrap()
            ),
            false => assert_ne!(
                expected_datetime,
                DatetimeValidator::validate(given_datetime).unwrap()
            ),
        };
    }
}
