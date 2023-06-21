#[cfg(test)]
mod turnaround_validator_tests {
    use due_date_calculator::validator::turnaround::TurnaroundValidator;
    use test_case::test_case;
    #[test_case(8; "Validate turnaround hours with validator, should work successfully")]
    #[test_case(11;)]
    #[test_case(1;)]
    #[test_case(3;)]
    fn test_turnaround_validator(hours: i64) {
        let _ = TurnaroundValidator::validate(hours);
    }
}
