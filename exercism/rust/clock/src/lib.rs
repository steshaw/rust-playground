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
    // Construct a new Clock from `hours` hours and `minutes` minutes.
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        hours += minutes / 60;
        minutes = minutes % 60;
        if minutes < 0 {
            hours -= 1;
            minutes += 60;
        }
        hours = hours % 24;
        if hours < 0 {
            hours += 24;
        };
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
