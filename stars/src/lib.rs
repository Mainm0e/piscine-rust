/* Instructions
Create a function named stars that takes a u32 as an argument and returns a String of stars (asterisks). The number of stars returned is 2^n (2 to the nth power).

Expected functions
pub fn stars(n: u32) -> String {

}
 */

 pub fn stars(n: u32) -> String {
    let num_stars = 2u32.pow(n);
    let stars = "*".repeat(num_stars as usize);
    stars
}