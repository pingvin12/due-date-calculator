use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalculationError {
    // error types used by the calculator struct
    // this type is not used at the moment, but when we switch from static input for workday length then we will need it
    #[error("Error: workday length is either not set or invalid.\nHint: set WORKDAY_LENGTH in .env variable as a number.\n (example: WORKDAY_LENGTH = 8)")]
    InvalidWorkdayLength,
    #[error("Error: unknown calculation error")]
    Unknown,
}
