// todo:division_and_remainder
/*Instructions
Create a function named divide that receives two i32 and returns a tuple. The first element is the result of the integer division between the two numbers, and the second is the remainder of the division.

pub fn divide(x: i32, y: i32) -> (i32, i32) {
} */

pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let division = x / y;
    let remainder = x % y;
    (division, remainder)
}
