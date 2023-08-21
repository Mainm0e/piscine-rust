// todo : name_initials
/* 
##Instructions
Create a function named initials. This function will receive a vector of string literals with names, and return a vector of Strings with the initials of each name.

Expected Functions
pub fn initials(names: Vec<&str>) -> Vec<String> {
}
Your heap allocations will be monitored to ensure that you do not make too many allocations, and that your allocations are reasonably sized. 
expect  James John == "J. J."*/

pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            let parts: Vec<&str> = name.split_whitespace().collect();
            match parts.len() {
                0 => String::new(),
                1 => {
                    if let Some(first_char) = parts[0].chars().next() {
                        format!("{}.", first_char.to_uppercase())
                    } else {
                        String::new()
                    }
                }
                _ => {
                    let mut initials = String::new();
                        for (i, part) in parts.iter().enumerate() {
                        if let Some(first_char) = part.chars().next() {
                             if i > 0 {
                                initials.push(' '); // Add space between initials
                            }
                            initials.push_str(&format!("{}.", first_char.to_uppercase()));
                        }
                    }
                    initials
                }
            }
        })
        .collect()
}