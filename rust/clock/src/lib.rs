use std::fmt;

#[derive (Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = (hours % 24 + minutes / 60) % 24;
        if hours < 0 { h += 24; }
        let mut m = minutes % 60;
        if minutes < 0 && minutes % 60 != 0 {
            h -= 1;
            m += 60;
        }
        if h < 0 { h += 24; }
        Clock { hours: h, minutes: m }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    // pub fn to_string(&self) -> String {
    //    format!("{:02}:{:02}", self.hours, self.minutes)
    // }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}