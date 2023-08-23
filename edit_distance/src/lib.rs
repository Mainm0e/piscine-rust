// todo:  edit_distance
/*
Instructions
Create a function named edit_distance, which calculates the minimum number of changes 
(insertions, deletions and/or substitutions) which are needed to transform the source string to the target string.

Expected Function
pub fn edit_distance(source: &str, target: &str) -> usize {
} */

// pub fn edit_distance(source: &str, target: &str) -> usize {
//     let mut source_chars = source.chars().collect::<Vec<char>>();
//     let mut target_chars = target.chars().collect::<Vec<char>>();
//     let mut distance = 0;
//     let mut i = 0;
//     let mut j = 0;
//     while i < source_chars.len() && j < target_chars.len() {
//         if source_chars[i] != target_chars[j] {
//             distance += 1;
//             if source_chars.len() > target_chars.len() {
//                 source_chars.remove(i);
//             } else if source_chars.len() < target_chars.len() {
//                 source_chars.insert(i, target_chars[j]);
//             } else {
//                 source_chars[i] = target_chars[j];
//             }
//         } else {
//             i += 1;
//             j += 1;
//         }
//     }
//     distance
// }

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
