// todo: middle_day
/* 
Instructions
Use the chrono crate to create a function named middle_day. It accepts a year, and returns the weekday of the middle day of that year, wrapped in an Option. chrono::Weekday has to be referred to as wd.

Years with an even number of days do not have a middle day, and should return None.

Expected Function
You'll need to work out the function signature for yourself.

Usage
Here is a program to test your function:

use middle_day::*;

fn main() {
    println!("{:?}", middle_day(1022).unwrap());
} */
use chrono::{Datelike, NaiveDate, Weekday};



pub fn middle_day(year: i32) -> Option<Weekday> {
    if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
        if leap_year(date.year()) {
            return None;
        } else {
            let middle = date + chrono::Duration::days(182);
            return Some(middle.weekday());
        }
    }
    None
}

fn leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            }
            return false;
        }
        return true;
    }
    false
}
