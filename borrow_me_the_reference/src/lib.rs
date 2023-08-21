// todo : borrow_me_the_reference
/*
Instructions
Ownership is Rust's most unique feature. It enables Rust to make memory safety guarantees without needing a garbage collector.

Understanding ownership is essential to take full advantage of Rust capabilities, it influences almost all aspects of the language.

Create the following functions:

delete_and_backspace: which receives a borrowed string, and processes it. - represents the backspace key and + represents the delete key, so that "helll-o" and "he+lllo" are both converted to "hello". The - and + characters should be removed from the string.

do_operations: which borrows a Vector of string literals representing simple addition and subtraction equations. The function should replace the operation with the result.

Expected Functions
pub fn delete_and_backspace(s: &mut String) {
}

pub fn do_operations(v: &mut Vec<String>) {
} */
//"bpp--o+er+++sskroi-++lcw"
// - means delete previous character
// + means delete next character
// but if - or + is next or previous character, increate or decrease the index by 1

pub fn delete_and_backspace(s: &str) -> String {
    let mut new_string = String::new();
    let mut i = 0;
    while i < s.len() {
        if s.chars().nth(i).unwrap() == '-' {
            if i > 0 {
                new_string.pop();
            }
            i += 1;
        } else if s.chars().nth(i).unwrap() == '+' {
            if i < s.len() - 1 {
                i += 1;
            }
        } else {
            new_string.push(s.chars().nth(i).unwrap());
            i += 1;
        }
    }
    new_string
}


pub fn do_operations(v: &mut Vec<String>) {
    for i in 0..v.len() {
        let mut j = 0;
        while j < v[i].len() {
            if v[i].chars().nth(j).unwrap() == '+' {
                let a = v[i].chars().take(j).collect::<String>().parse::<i32>().unwrap();
                let b = v[i].chars().skip(j + 1).collect::<String>().parse::<i32>().unwrap();
                v[i] = (a + b).to_string();
                break;
            } else if v[i].chars().nth(j).unwrap() == '-' {
                let a = v[i].chars().take(j).collect::<String>().parse::<i32>().unwrap();
                let b = v[i].chars().skip(j + 1).collect::<String>().parse::<i32>().unwrap();
                v[i] = (a - b).to_string();
                break;
            } else {
                j += 1;
            }
        }
    }
}
