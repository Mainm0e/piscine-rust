// todo: find_factorial 
/* 
Instructions
Create a function named factorial which returns the factorial of a given number.

pub fn factorial(num: u64) -> u64 {
}
As a reminder, the factorial of a number is the product of all the integers from 1 to that number.

Example: the factorial of 6 (written 6!) is 1 * 2 * 3 * 4 * 5 * 6 = 720.

Do not forget the rules for 0 and 1.
 */

 pub fn factorial(num: u64) -> u64 {
    let mut result = 1;
    for i in 1..=num {
        // ?? example num = 5 : 1 * 1 = 1, 1 * 2 = 2, 2 * 3 = 6, 6 * 4 = 24, 24 * 5 = 120
        result *= i;
    }
    result
}