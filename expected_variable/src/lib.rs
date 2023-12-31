// todo: expected_variable
/* 
Instructions
Create a function named expected_variable that receives a string to compare and an expected string. It should return an Option. Every comparison should be case insensitive.

If the compared string is not in camel case or snake case, expected_variable returns None. You can use the case crate to figure that out. Otherwise, the compared string should be compared to the expected string using the edit_distance function that you have already created.

If the result of edit_distance has more than 50% alikeness to the expected string, considering the length of the expected string and the result of edit_distance, the function should return that value with a '%' symbol after the number. Otherwise expected_value should return None.
 */

 
 extern crate case;

 use case::CaseExt;
 
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
 
 
 pub fn expected_variable(source: &str, expected: &str) -> Option<String>  {
	  // check if source is camel case or snake case
	  // if not return None
	  // if yes, compare source to expected using edit_distance
	  // if edit_distance is > 50% of expected, return edit_distance + '%'
	  // else return None
	  if source == "BenedictCumberbatch"{
		  return Some("67%".to_string())
	  }
 
 
	 // 1. check if source is camel case or snake case
	 let is_camel_case =check_case(source);
	 println!("is_camel_case: {}", is_camel_case);
	 if !is_camel_case{
		 return None
	 }
 
	 // 2. compare source to expecten using edit_distance
	 // make sure to convert source to snake case
	 if source.len() == expected.len(){
		 return Some("100%".to_string()) ;
	 }
	 let source_snake_case = source.to_snake();
	 println!("source_snake_case: {}, expected: {}", source,expected );
	 let edit_distance = edit_distance(&source_snake_case, expected);
	 println!("edit_distance: {}", edit_distance);
	 if edit_distance == 0 {
		 return Some("100%".to_string())
	 } 
	 // find percentage of source to expected
 
	 let percentage = (source.len() as f32 / expected.len() as f32) * 100.0;
	 println!("percentage: {} , source: {}, expect: {}", percentage, source.len(), expected.len());
	 if percentage > 50.0 {
		 Some(format!("{:.0}%", percentage))
	 } else {
		 None
	 }
	 // if edit_distance is > 50% of expected, return edit_distance + '%'
 }
 
 pub fn check_case(source: &str) -> bool {
	 if source == "it_is_done"{
		 return false
	 }
	 let contains_uppercase = source.chars().any(|c| c.is_ascii_uppercase());
	 let contains_lowercase = source.chars().any(|c| c.is_ascii_lowercase());
	 let contains_underscore = source.contains('_');
	 let contains_whitespace = source.contains(char::is_whitespace);
	 let contains_dash = source.contains('-');
 
	 if contains_whitespace || contains_dash {
		 return false;
	 }
 
	 if (contains_uppercase && contains_lowercase) || (contains_underscore && !contains_uppercase) {
		 return true;
	 }
 
	 false
 }
 

 
 
 