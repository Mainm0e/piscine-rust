// todo: panic
/* 
Instructions
Create a function that tries to open a file and panics if the file does not exist.

Expected Function
pub fn open_file(s: &str) -> File {

} */

use std::fs::File;
pub fn open_file(s: &str) -> File {
    File::open(s).expect("File not found")
}