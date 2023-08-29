// todo: pangram
/* 
Instructions
Create a function named is_pangram which returns true if the given string is a pangram.

A pangram is a sentence which uses every letter of the alphabet at least once.

Example: "The quick brown fox jumps over the lazy dog."

Expected functions
pub fn is_pangram(s: &str) -> bool {

} */

pub fn is_pangram(s: &str) -> bool {
    let mut alphabet = [false; 26];
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            alphabet[(c.to_ascii_lowercase() as u8 - b'a') as usize] = true;
        }
    }
    alphabet.iter().all(|&x| x)

}

