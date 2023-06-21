use crate::misc::errortypes::validation_error::ValidationError;

pub struct TurnaroundValidator {}

impl TurnaroundValidator {
    // validates hours by checking if it is larger than 0, mainly useful for input and preventing stackoverflow
    pub fn validate(hours: i64) -> Result<(), ValidationError> {
        if hours < 0 {
            return Err(ValidationError::InvalidTurnaround {
                example: "8".to_owned(),
                found: hours.to_string(),
            });
        }
        Ok(())
    }
}
