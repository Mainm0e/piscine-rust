// todo: arrays
/* Instructions
Define a function named thirtytwo_tens that returns an array with 32 positions filled with only the value 10, so that [10, 10, 10, ... 10].len() is equal to 32.

Write a function that takes an array of i32 and returns the sum of the elements (make it work with the main).

Expected functions
The type of one of the arguments is missing. Use the example main function to determine the correct type.

pub fn sum(a: _) -> i32 {
	//type of argument missing in the signature here
}

pub fn thirtytwo_tens() -> [i32; 32] {
} */


// solution
pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}
