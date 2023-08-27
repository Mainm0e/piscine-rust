// todo: handling
/* 
Instructions
Create a function named open_or_create which has two arguments:

file : &str: which represents a file path.
content: &str which will be the content to be written to the file.
This function should try to open a file. If it does not exist, the file should be created. In case something goes wrong, it should panic, with the error.

Expected Function
pub fn open_or_create(file: &str, content: &str) {
}
 */

 use std::fs::File;
 use std::io::prelude::*;

 
 pub fn open_or_create(file_path: &str, content: &str) {
     let mut file = match File::create(file_path) {
         Ok(file) => file,
         Err(_) => File::create(file_path).expect("Failed to create the file"),
     };
     file.write_all(content.as_bytes()).expect("Failed to write to the file");
 }
 