use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = minutes % 60;
        let rem_h = (minutes as f64 / 60 as f64).floor() as i32;
        let h = (hours + rem_h) % 24;
        let m = if m < 0 { m + 60 } else { m };
        let h = if h < 0 { h + 24 } else { h };
        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
