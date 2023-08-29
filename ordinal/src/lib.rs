// todo:  ordinal
/* 
Instructions
Complete the function num_to_ordinal. It returns the ordinal number for a given cardinal number.

Expected functions
pub fn num_to_ordinal(x: u32) -> String {

} */

pub fn num_to_ordinal(x: u32) -> String {
    let mut s = x.to_string();
    let last = s.pop().unwrap();
    let second_last = s.pop().unwrap_or('0');
    let suffix = match (second_last, last) {
        ('1', _) => "th",
        (_, '1') => "st",
        (_, '2') => "nd",
        (_, '3') => "rd",
        _ => "th",
    };
    format!("{}{}", x, suffix)

}