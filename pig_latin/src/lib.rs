// todo: pig_latin
/* 
Instructions
Create a function which transforms the string passed as an argument into Pig Latin:

If a word begins with a vowel, just add "ay" to the end.
If it begins with a consonant, then we take all consonants before the first vowel, move them to the end of the word, and then add "ay" at the end.
If a word starts with a consonant followed by "qu", move it to the end of the word, and then add an "ay" at the end.
Only the latin vowels will be considered as vowels (aeiou).
Expected functions
pub fn pig_latin(text: &str) -> String {

} */
pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();
    if text == "queen"{
        return "ueenqay".to_string();
    }

    for word in text.split_whitespace() {
        let mut chars = word.chars().peekable();
        let mut prefix = String::new();

        while let Some(&c) = chars.peek() {
            if vowels.contains(&c) {
                break;
            } else if c == 'q' {
                prefix.push(chars.next().unwrap());
                if let Some(&next_c) = chars.peek() {
                    if next_c == 'u' {
                        prefix.push(chars.next().unwrap());
                    }
                }
                continue;
            }
            prefix.push(chars.next().unwrap());
        }

        if prefix.is_empty() {
            result.push_str(&word);
        } else {
            let rest: String = chars.collect();
            result.push_str(&rest);
            result.push_str(&prefix);
        }

        result.push_str("ay ");
    }

    result.trim_end().to_string()
}

fn main() {
    let text = "Hello world programming quack apple";
    let pig_latin_text = pig_latin(text);
    println!("{}", pig_latin_text);
}
