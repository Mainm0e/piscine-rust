// todo : ownership
/*
Instruction
Create a function named first_subword, that takes ownership of a string 
and returns the first sub-word in it. 
It should work for camelCase, PascalCase, and snake_case.

Expected functions
pub fn first_subword(mut s: String) -> String {
}
 */


pub fn first_subword(mut s: String) -> String {
    let mut result = String::new();
    let mut new_word = true; // Flag to identify the start of a new word

    for c in s.chars() {
        if c.is_uppercase() {
            if !new_word {
                break;
            }
        } else if c == '_' {
            break;
        }

        result.push(c);
        new_word = false;
    }
    result
}
    