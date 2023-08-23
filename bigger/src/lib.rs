// todo: bigger
/* 
Instructions
Create a function named bigger that gets the biggest positive number in the HashMap.

Expected Function
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
} */

use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut biggest = 0;
    for (_, value) in h {
        if value > biggest {
            biggest = value;
        }
    }
    biggest
}