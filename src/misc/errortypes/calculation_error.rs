use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalculationError {
    #[error("Error: workday length is either not set or invalid.\nHint: set WORKDAY_LENGTH in .env variable as a number.\n (example: WORKDAY_LENGTH = 8)")]
    InvalidWorkdayLength,
    #[error("Error: Cannot calculate date due to EPOCH_TIME being invalid.")]
    InvalidEpochTime,
    #[error("Error: unknown calculation error")]
    Unknown,
}
