// todo: expected_variable
/* 
Instructions
Create a function named expected_variable that receives a string to compare and an expected string. It should return an Option. Every comparison should be case insensitive.

If the compared string is not in camel case or snake case, expected_variable returns None. You can use the case crate to figure that out. Otherwise, the compared string should be compared to the expected string using the edit_distance function that you have already created.

If the result of edit_distance has more than 50% alikeness to the expected string, considering the length of the expected string and the result of edit_distance, the function should return that value with a '%' symbol after the number. Otherwise expected_value should return None.
 */


 pub fn edit_distance(source: &str, target: &str) -> usize {
	let src = source.chars().collect::<Vec<_>>();
	let tar = target.chars().collect::<Vec<_>>();
	let source_len = src.len() + 1;
	let target_len = tar.len() + 1;

	if source_len == 0 {
		return target_len;
	}
	if target_len == 0 {
		return source_len;
	}

	let mut matrix = vec![vec![0; source_len]; target_len];

	for i in 1..target_len {
		matrix[i][0] = i
	}
	for j in 1..source_len {
		matrix[0][j] = j
	}

	for i in 1..target_len {
		for j in 1..source_len {
			let x = if src[j - 1] == tar[i - 1] {
				matrix[i - 1][j - 1]
			} else {
				1 + std::cmp::min(
					std::cmp::min(matrix[i][j - 1], matrix[i - 1][j]),
					matrix[i - 1][j - 1],
				)
			};
			matrix[i][j] = x;
		}
	}
	matrix[target_len - 1][source_len - 1]
}

use std::cmp::Ordering;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Check if the compared string is in camel case or snake case
    if is_snake_case(compared) || is_camel_case(compared) {
        // Calculate the edit distance
        let distance = edit_distance(compared, expected);

        // Calculate the similarity percentage
        let similarity = 1.0 - (distance as f64 / expected.len() as f64);

        // Check if similarity is greater than 50%
        if similarity > 0.5 {
            // Format the result and return
            let formatted_result = format!("{:.0}%", similarity * 100.0);
            Some(formatted_result)
        } else {
            None
        }
    } else {
        None
    }
}

fn is_snake_case(s: &str) -> bool {
    // Implement snake case check logic here
    // For example, check if the string only contains lowercase letters and underscores
    s.chars().all(|c| c.is_ascii_lowercase() || c == '_')
}
fn is_camel_case(s: &str) -> bool {
    let mut has_upper = false;
    let mut has_lower = false;

    for c in s.chars() {
        if c.is_ascii_uppercase() {
            has_upper = true;
        } else if c.is_ascii_lowercase() {
            has_lower = true;
        }
    }

    has_upper && has_lower
}
