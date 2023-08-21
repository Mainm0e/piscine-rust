// todo : reverse_string
/* 
Instructions
Create a function named rev_str that takes a &str as a parameter, and returns a String with its letters reversed.

*/
pub fn rev_str(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars().rev() {
        result.push(c);
    }
    result
}

