// todo: string_permutation
/* 
Instructions
Define the function is_permutation, that returns true if the string s1 is a permutation of s2.

s1 is a permutation of s2 if all the elements in s1 appear the same number of times in s2, and all the characters in s1 appear in s2 even if they are in different order.

Expected Function
pub fn is_permutation(s1: &str, s2: &str) -> bool {
} */
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1 = s1.chars().collect::<Vec<char>>();
    let mut s2 = s2.chars().collect::<Vec<char>>();
    s1.sort();
    s2.sort();
    s1 == s2
}