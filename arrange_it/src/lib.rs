// todo: arrange_it
/* 
Instructions
Create a function named arrange_phrase, that takes a string literal, sorts the words and returns it. Each word will contain a number that indicates the position of that word.

Expected Functions
pub fn arrange_phrase(phrase: &str) -> String {
}
Your heap allocations will be monitored to ensure that you do not make too many allocations, and that your allocations are reasonably sized. */


pub fn arrange_phrase(phrase: &str) -> String {

    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    words.sort_by_key(|word| word.chars().find(|c| c.is_numeric()).unwrap());
    words.join(" ");
    // find the number in each word and delete it

    // loop all chars in the string and find the number in each word and delete it
     let mut new2_words: Vec<&str> = words;
     let mut result = String::new();
        for word in new2_words {
            let mut chars = word.chars();
            let mut new_word = String::new();
            while let Some(c) = chars.next() {
                if c.is_numeric() {
                    continue;
                }
                new_word.push(c);
            }
            result.push_str(&new_word);
            result.push(' ');
        }
        result.pop();
        result


}
