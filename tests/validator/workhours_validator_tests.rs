#[cfg(test)]
mod workhours_validator_tests {
    use due_date_calculator::validator::{localdatetime::DatetimeValidator, workhours::WorkhoursValidator};
    use test_case::test_case;
    use std::time::{Duration, SystemTime};
    
    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1687257000); "Validate workhour to verify it is inside working hours, should work succesfully")]
    fn test_workhour_validator(datetime: SystemTime) {
        WorkhoursValidator::validation(datetime);
    }
}