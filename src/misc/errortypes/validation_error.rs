use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    // error types used by validator structs
    #[error("Error while parsing syntax from local_date_time.\nAre you sure it is a valid date/time format?\n (example: 2023-06-20T10:30:00Z, found: `{found:?}`")]
    InvalidDateSyntax { found: String },
    #[error("Error: Invalid turnaround time given from .env file.\nAre you sure turnaround time is the correct format?\n (example: `{example:?}`, found: `{found:?}`")]
    InvalidTurnaround { example: String, found: String },
    #[error("Error: Invalid working hours, or date must be within working hours!\nAre you sure the date/time is within working hours?\n (example: `{example:?}, found: `{found:?}`")]
    InvalidWorkingHours { example: String, found: String },
    #[error("Error: Date is not inside available working days,\n Are you sure the date is within available working days?\n (found: `{found:?}`")]
    InvalidWorkingDaysDate { found: String },
    #[error("Error: Unknown validation error")]
    Unknown,
}
