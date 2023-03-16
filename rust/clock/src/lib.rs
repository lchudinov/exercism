use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;
        let mut minutes = minutes;

        while minutes < 0 {
            hours = hours - 1;
            minutes = minutes + 60;
        }

        while minutes > 59 {
            hours = hours + 1;
            minutes = minutes - 60;
        }
    
        while hours < 0 {
            hours = hours + 24
        }
    
        while hours > 23 {
            hours = hours - 24
        }
    
        Clock{hours, minutes}
    }
    
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours = self.hours;
        let minutes = self.minutes + minutes;
        Self::new(hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}