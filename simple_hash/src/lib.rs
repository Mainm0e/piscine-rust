// todo: simple_hash
/* 
Instructions
Create a function named word_frequency_counter which will receive a vector of strings (each string being a single word) and return an HashMap with the word as the key and the number of repetitions as the value.

Also create a function named nb_distinct_words which will take a reference to the HashMap and return the number of distinct words present in it.

all the words tested will be lowercase

Expected functions
pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {}
 */

use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut map = HashMap::new();
    for word in words {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}