pub mod calculator;
pub mod validator {
    pub mod localdatetime;
    pub mod turnaround;
    pub mod workhours;
}

pub mod misc {
    pub mod datetime_utils;
    pub mod errortypes {
        pub mod calculation_error;
        pub mod validation_error;
    }
}
