use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours % 24, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Construct a new Clock from `hours` hours and `minutes` minutes.
        let hours = if minutes < 0 {
            hours - 1
        } else {
            hours + minutes / 60
        };
        let hours = if hours < 0 { hours % 24 + 24 } else { hours };
        let minutes = if minutes < 0 {
            minutes % 60 + 60
        } else {
            minutes % 60
        };
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        todo!("Add {minutes} minutes to existing Clock time");
    }
}
