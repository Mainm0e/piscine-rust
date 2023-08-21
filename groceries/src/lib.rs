/*
Description: 
  Instructions
Create a function named insert, that inserts a new element at the end of the Vec.

Create another function named at_index that returns the value found at the index passed as an argument.
 */

pub fn insert(vec: &mut Vec<String>, val: String) {
vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
vec[index].clone()
}