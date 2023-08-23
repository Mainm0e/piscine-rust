// todo: strings 
/* 

Instructions
Create a function which receives a string slice and returns the number of characters in that string.

Expected Function
pub fn char_length(s: &str) -> usize {
} */

pub fn char_length(s: &str) -> usize {
    s.chars().count()
}