use std::time::{Duration, SystemTime};

use crate::misc::{
    datetime_utils::DatetimeUtils,
    datetime_utils::{EligibleDays, WORKDAY_LENGTH},
    errortypes::calculation_error::CalculationError,
};

pub struct Calculator {
    workday_length: i16,
}

impl Calculator {
    // retrieve workday length thats static and store it for ease of access
    pub fn new() -> Result<Self, CalculationError> {
        Ok(Calculator {
            workday_length: WORKDAY_LENGTH.to_owned().try_into().unwrap(),
        })
    }

    //main entry function used after validating input
    pub fn calculate_due_date(
        &mut self,
        creation_date: SystemTime,
        turnaround_hours: i16,
    ) -> Result<SystemTime, CalculationError> {
        let mut workdays = self.get_work_days(turnaround_hours);

        let mut due_date = creation_date;
        while workdays > 0 {
            due_date = self.get_next_work_day(due_date)?;
            workdays = workdays - 1;
        }
        // add hours_in_workday to due_date
        let seconds = turnaround_hours as u64 * 60 * 60;
        if seconds > 0 {
            let duration = Duration::from_secs(seconds);
            due_date = due_date + duration;
            Ok(due_date)
        } else {
            Err(CalculationError::Unknown)
        }
    }

    // When given a SystemTime, returns a Result containing an enum EligibleDays for readability
    fn get_next_work_day(&mut self, datetime: SystemTime) -> Result<SystemTime, CalculationError> {
        let day_of_week = DatetimeUtils::get_day_of_week(datetime);
        if day_of_week == EligibleDays::Friday {
            return Ok(DatetimeUtils::add_days(datetime, 3));
        } else if day_of_week == EligibleDays::Saturday {
            return Ok(DatetimeUtils::add_days(datetime, 2));
        } else {
            return Ok(DatetimeUtils::add_days(datetime, 1));
        }
    }

    //Returns work days by subtracking the hours with workday_length. ex.: 16 / 2 = 8
    fn get_work_days(&mut self, hours: i16) -> i16 {
        return hours / self.workday_length;
    }
}

// Default implementation for calculator incase of getting rid of static inputs.
impl Default for Calculator {
    fn default() -> Self {
        Calculator { workday_length: 8 }
    }
}
