/*
sum() is a function that returns the sum of an addition between two unsigned
8-bit integers, and returns an unsigned 8-bit integer.
    - ie. the range of values for the input and return values is 0 to 255
*/
// * The first line in a function without a semicolon is the return value
pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

/*
diff() is a function that returns the difference of a subtraction between two
signed 16-bit integers, and returns a signed 16-bit integer.
    - ie. the range of values for the input and return values is -32,768 to
      32,767
*/
pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}
/*
pro() is a function that returns the product of a multiplication between two
signed 8-bit integers, and returns a signed 8-bit integer.
    - ie. the range of values for the input and return values is -128 to 127
*/
pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

/*
guo() is a function that returns the quotient of a division between two
signed 32-bit integers, and returns a signed 32-bit integer.
    - ie. the range of values for the input and return values is -2,147,483,648
      to 2,147,483,647
*/
pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

/*
rem() is a function that returns the remainder of a division between two
32-bit values, and returns a 32-bit value.
    - ie. the range of values for the input and return values a floating point
      number 
*/
pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

#[cfg(test)]
mod tests_scalar1;
