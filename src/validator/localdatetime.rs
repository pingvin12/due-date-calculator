use std::{str::FromStr, time::SystemTime};

use chrono::{DateTime, Utc};

use crate::misc::errortypes::validation_error::ValidationError;

pub struct DatetimeValidator {}

impl DatetimeValidator {
    pub fn validate(datetime: &str) -> Result<SystemTime, ValidationError> {
        let var = match DateTime::<Utc>::from_str(datetime) {
            Ok(system_time) => system_time,
            Err(_) => {
                return Err(ValidationError::InvalidDateSyntax {
                    found: datetime.to_owned(),
                })
            }
        };

        Ok(Into::<SystemTime>::into(var))
    }
}
