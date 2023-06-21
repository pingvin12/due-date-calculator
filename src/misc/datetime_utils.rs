use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::HashMap;
use chrono::{DateTime, Local};
use lazy_static::lazy_static;

// This enum exists because it is nicer to look at than looking at a i8 variable
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EligibleDays {
    Sunday, //0
    Monday, //1
    Tuesday, //2
    Wednesday, //3
    Thursday, //4
    Friday, //5
    Saturday, //6
}

// Store every information we need staticly
lazy_static! {
    pub static ref DAYS_MAP: HashMap<u8, Arc<EligibleDays>> = {
        let mut map = HashMap::new();
        map.insert(0, Arc::new(EligibleDays::Sunday));
        map.insert(1, Arc::new(EligibleDays::Monday));
        map.insert(2, Arc::new(EligibleDays::Tuesday));
        map.insert(3, Arc::new(EligibleDays::Wednesday));
        map.insert(4, Arc::new(EligibleDays::Thursday));
        map.insert(5, Arc::new(EligibleDays::Friday));
        map.insert(6, Arc::new(EligibleDays::Saturday));

        map
    };

    pub static ref WORKINGHOURS_BEFORE: i8 = 9;
    pub static ref WORKINGHOURS_AFTER: i8 = 17;
    pub static ref WORKDAY_LENGTH: i8 = 8;
}

pub struct DatetimeUtils {}

impl DatetimeUtils {
    pub fn get_day_of_week(datetime: SystemTime) -> EligibleDays {
        let duration = match datetime.duration_since(UNIX_EPOCH) {
            Ok(next) => next.as_secs() / (24 * 60 * 60),
            Err(e) => panic!("{}", e),
        };

        // Assuming Sunday is represented by 0
        let epoch: u8 = (duration % 7).try_into().unwrap();
        let day_of_the_week = DAYS_MAP.iter().find(|(k, _)| **k == epoch);

        day_of_the_week.unwrap().1.as_ref().clone()
    }

    pub fn has_surpassed_hours(date: SystemTime, hours: u8) -> bool {
        let start_of_day = {
            let mut date = date;
            date = date - Duration::from_secs(date.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() % (24 * 60 * 60));
            date + Duration::from_secs(8 * 60 * 60)
        };

        let duration = match date.duration_since(start_of_day) {
            Ok(dur) => dur,
            Err(_) => return false, // date is earlier than start_of_day
        };

        let seconds_passed = duration.as_secs();
        let hours_passed = seconds_passed / 3600;

        hours_passed >= u64::from(hours)
    }

    pub fn format_system_time(system_time: SystemTime) -> String {
        let local_datetime: DateTime<Local> = system_time.into();
        let formatted_time = local_datetime.format("%Y-%m-%d %H:%M:%S");
        formatted_time.to_string()
    }
    // Adding days to system_time
    pub fn add_days(system_time: SystemTime, days: u64) -> SystemTime {
        let duration = Duration::from_secs(days * 24 * 60 * 60);
        system_time + duration
    }
}
