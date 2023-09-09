/* slices_to_map
Instructions:
Create a function that borrows two slices and returns a hashmap where the first slice represents the keys and the second represents the values.

If the slices have different sizes, the function should return the hashmap with the size of the smallest list.
Expected Function
pub fn slices_to_map(&[T], &[U]) -> HashMap<&T, &U> {

}
Usage
Here is a program to test your function.

use slices_to_map::*;

fn main() {
	let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
	let values = [1, 3, 23, 5, 2];
	println!("{:?}", slices_to_map(&keys, &values));
}
And its output

$ cargo run
{"James": 2, "Liam": 3, "Emma": 23, "Noah": 5, "Olivia": 1}
$ */



use std::collections::HashMap;

pub fn slices_to_map<'a, T, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U>
where
    T: std::hash::Hash + Eq,
{
    let min_len = std::cmp::min(keys.len(), values.len());
    let mut map = HashMap::with_capacity(min_len);

    for i in 0..min_len {
        map.insert(&keys[i], &values[i]);
    }

    map
}
