// todo: string literals
/*
Instructions
Create the following functions:

is_empty: that returns true if the string is empty.
is_ascii: that returns true if all characters are within the ASCII range.
contains: that returns true if the string contains the given pattern.
split_at: that divides a string in two returning a tuple.
find: that returns the index of the first character of a given string that matches the pattern.
Expected functions
pub fn is_empty(v: &str) -> bool {
}

pub fn is_ascii(v: &str) -> bool {
}

pub fn contains(v: &str, pat: &str) -> bool {
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
}

pub fn find(v: &str, pat: char) -> usize {
}
Your heap allocations will be monitored to ensure that you do not make too many allocations, and that your allocations are reasonably sized. */

pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap_or(v.len())
}

