
// todo:capitalizing
/* Instructions
Complete the capitalize_first function which converts the first letter of the string to uppercase.

Complete the title_case function which converts the first letter of each word in a string to uppercase.

Complete the change_case function which converts the uppercase letters of a string into lowercase, and the lowercase to uppercase.

Expected Functions
pub fn capitalize_first(input: &str) -> String {
}

pub fn title_case(input: &str) -> String {
}

pub fn change_case(input: &str) -> String {
} */

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    for word in input.split_whitespace() {
        result.push_str(&capitalize_first(word));
        result.push(' ');
    }
    result.trim().to_string()
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_lowercase() {
            result.push(c.to_uppercase().next().unwrap());
        } else {
            result.push(c.to_lowercase().next().unwrap());
        }
    }
    result
}