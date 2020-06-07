use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes = self.minutes + minutes;
        *self
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            format!("{:02}", self.hours),
            format!("{:02}", self.minutes)
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clock")
            .field("minutes", &self.minutes)
            .field("hours", &self.hours)
            .finish()
    }
}

impl Copy for Clock {}

impl Clone for Clock {
    fn clone(&self) -> Clock {
        *self
    }
}
