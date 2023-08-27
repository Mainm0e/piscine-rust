// todo:  cipher
/* 
Instructions
The Atbash cipher is an encryption method in which each letter of a word is replaced by its mirror letter in the alphabet.

Your objective is to create a function named cipher which must return a Result wrapped in an Option. The Result should contain either a boolean or an Error based on the CipherError structure. This structure should be the error type for the function cipher.

cipher should compare the original String with the ciphered String. It should return true if the cipher is correct. If the cipher is incorrect it should return the error type CipherError with a boolean and the expected atbash cipher String.

Expected Function and structure
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    // expected public fields
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {

    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {

} */
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}


pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }
    
    let mut ciphered_string = String::new();
    for c in original.chars() {
        if c.is_alphabetic() {
            let offset = c as u8 - if c.is_uppercase() { b'A' } else { b'a' };
            let ciphered_char = ((25 - offset) + if c.is_uppercase() { b'A' } else { b'a' }) as char;
            ciphered_string.push(ciphered_char);
        } else {
            ciphered_string.push(c);
        }
    }
    
    Some(if ciphered_string == ciphered {
        Ok(true)
    } else {
        Err(CipherError::new(false, ciphered_string))
    })
}

