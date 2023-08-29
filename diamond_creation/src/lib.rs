// todo: diamond_creation
/* Instructions
Build the function make_diamond which takes a letter as an input, and returns a diamond.

Rules:

The first and last row contain one 'A'.
The given letter has to be at the widest point.
All rows, except the first and last, have exactly two identical letters.
All rows have as many trailing spaces as leading spaces. This may be 0.
The diamond is vertically symmetrical, and horizontally symmetrical.
The width of the diamond equals its height.
The top half has letters in ascending order (abcd).
The bottom half has letters in descending order (dcba).
Expected functions
pub fn get_diamond(c: char) -> Vec<String> {

} */
pub fn get_diamond(c: char) -> Vec<String> {
    // Find the number of rows
    let rows = (c as u8 - b'A') * 2 + 1;
    // Create a vector of strings
    let mut diamond: Vec<String> = Vec::new();
    
    // Loop from char A to the given char
    for (index, ch) in (b'A'..=c as u8).enumerate() {
        // Create a string of the current char
        let mut row = String::new();
        
        // Calculate the number of spaces on both sides of the char
        let side_spaces = (rows - 2 * index as u8 - 1) / 2;
        
        // Add leading spaces
        row.push_str(&" ".repeat(side_spaces as usize));
        
        // Add the current character
        row.push(ch as char);
        
        // Add spaces between mirrored characters
        if index > 0 {
            let mid_spaces = (2 * index - 1) as usize;
            row.push_str(&" ".repeat(mid_spaces));
            row.push(ch as char);
        }
        
        // Add trailing spaces
        row.push_str(&" ".repeat(side_spaces as usize));
        diamond.push(row);
    }
    let mut mirrored_vector: Vec<String> = diamond
    .iter()
    .map(|s| s.chars().rev().collect())
    .collect();
    let mut count = 0;
    for row in diamond.iter().skip(0).rev() {
        if count == 0 {
            count += 1;
            continue;
        }
        mirrored_vector.push(row.chars().rev().collect());
    }
    mirrored_vector
}


/* pub fn get_diamond(c: char) -> Vec<String> {
    // find the number of rows
    let rows_length = (c as u8 - b'A') * 2 + 1;
    // find the number of columns
    println!("rows: {}", rows);
    let cols = rows;
    // create a vector of strings
    let mut diamond: Vec<String> = Vec::new();
    
    // loop from char A to the given char
    // ····A····
    // ···B·B···
    // ··C···C··
    for (index, ch) in (b'A'..=c as u8).enumerate() {
        // create a string of the current char
        let mut row = String::new();
        // get row length then make division by 2 add ch to row like mirror
        //like this ····A····




        // create
        diamond.push(row);
    }


    diamond


} */