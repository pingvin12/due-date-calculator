#[cfg(test)]
mod workhours_validator_tests {
    use due_date_calculator::validator::workhours::WorkhoursValidator;
    use std::time::{Duration, SystemTime};
    use test_case::test_case;

    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1687257000); "Validate workhour to verify it is inside working hours, should work succesfully")]
    fn test_workhour_validator(datetime: SystemTime) {
        WorkhoursValidator::validation(datetime);
    }
}
