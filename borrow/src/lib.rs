// todo: borrow
/*
### Instructions

Create a function named str_len, you'll need to complete the function signature. Your function should accept a string or a string literal, and return its length without taking ownership of the value (i.e, borrowing the value).

Expected functions
pub fn str_len(s: ) -> usize {
}

*/

pub fn str_len(s: &str) -> usize {
    s.len()
}


