use std::{sync::Arc, time::SystemTime};

use chrono::{DateTime, Local};

use crate::misc::{
    datetime_utils::{DatetimeUtils, DAYS_MAP, WORKINGHOURS_AFTER, WORKINGHOURS_BEFORE},
    errortypes::validation_error::ValidationError,
};

pub struct WorkhoursValidator {
    pub eligible_workdays: DAYS_MAP,
}

impl WorkhoursValidator {
    pub fn validation(datetime: SystemTime) {
        let _ = Self::created_during_workhours(datetime);
        let _ = Self::created_on_workday(datetime);
    }

    // Checks that the datetime given is on a workday by comparing the day of the week with a static hashmap that contains the days of the week
    fn created_on_workday(datetime: SystemTime) -> Result<(), ValidationError> {
        let dt: DateTime<Local> = datetime.into();
        let day = DatetimeUtils::get_day_of_week(datetime);
        //find item from hashmap
        let dur = DAYS_MAP.iter().find(|(_, v)| **v == Arc::new(day.clone()));

        if let Some((_, _)) = dur {
            return Ok(());
        } else {
            // throw error if somehow the day doesnt exist in the hashmap
            return Err(ValidationError::InvalidWorkingDaysDate {
                found: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            });
        }
    }

    // checks the given datetime and throws an error if not inside workhours
    fn created_during_workhours(datetime: SystemTime) -> Result<(), ValidationError> {
        let dt: DateTime<Local> = datetime.into();
        let before: u8 = WORKINGHOURS_BEFORE.to_owned().try_into().unwrap();
        let after: u8 = WORKINGHOURS_AFTER.to_owned().try_into().unwrap();

        if DatetimeUtils::has_surpassed_hours(datetime, before)
            && !DatetimeUtils::has_surpassed_hours(datetime, after)
        {
            Ok(())
        } else {
            Err(ValidationError::InvalidWorkingDaysDate {
                found: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
        }
    }
}
