use std::fmt;
use std::fmt::Formatter;
use std::iter::successors;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
    
}

fn to_non_negative_num_of_minutes(mins: i32) -> i32 {
    successors(Some(mins), |el| Some(el + 1440)).skip_while(|h| h.is_negative()).next().unwrap()
}



impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let num_of_mins = to_non_negative_num_of_minutes(hours * 60 + minutes);
        let num_of_hours = num_of_mins / 60;
        let remaining_mins = num_of_mins % 60;
        Clock { hours: num_of_hours % 24, minutes: remaining_mins}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        todo!("Add {minutes} minutes to existing Clock time");
    }
}
