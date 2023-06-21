#[cfg(test)]
mod duedate_tests {
    use due_date_calculator::{calculator::Calculator, misc::datetime_utils::DatetimeUtils};
    use std::time::{Duration, SystemTime};
    use test_case::test_case;

    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1667320800), 1, SystemTime::UNIX_EPOCH + Duration::from_secs(1667324400); "test 1: When calculate due date returns success")]
    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1667320800), 0, SystemTime::UNIX_EPOCH + Duration::from_secs(1667320800); "test 2: When calculate due date with zero turnaround hours")]
    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1667320800), 24, SystemTime::UNIX_EPOCH + Duration::from_secs(1667839200); "test 3: When calculate due date with one day turnaround")]
    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1667320800), 48, SystemTime::UNIX_EPOCH + Duration::from_secs(1668357600); "test 4: When calculate due date with two days turnaround")]
    #[test_case(SystemTime::UNIX_EPOCH + Duration::from_secs(1667320800), 168, SystemTime::UNIX_EPOCH + Duration::from_secs(1670604000); "test 5: When calculate due date with one week turnaround")]
    fn test_calculate_due_date(
        creation_date: SystemTime,
        turnaround_hours: i16,
        duedate: SystemTime,
    ) {
        let mut calc = Calculator::new().unwrap();
        let res = calc.calculate_due_date(creation_date, turnaround_hours);
        if let Ok(value) = res {
            let fmt = format!("{}", DatetimeUtils::format_system_time(value));
            let fmt_expected = format!("{}", DatetimeUtils::format_system_time(duedate));
            assert_eq!(fmt_expected, fmt);
        }
    }
}
