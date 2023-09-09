/* closures
Instructions
Using closures and iterators create a function, that returns the first 50 even numbers squared in a Vec<i32>.

Expected Functions
fn first_fifty_even_square() -> Vec<i32> {

}
Usage
Here is a program to test your function.

use closures::*;

fn main() {
	println!("Hello, world!");
	let v1 = first_fifty_even_square();

	println!("All elements in {:?}, len = {}", v1, v1.len());
}
And its output:

$ cargo run
All elements in [4, 16, 36, ..., 10000], len = 50
$ */
/* 
    #[test]
    fn test() {
        let v1 = vec![
            4, 16, 36, 64, 100, 144, 196, 256, 324, 400, 484, 576, 676, 784, 900, 1024, 1156, 1296,
            1444, 1600, 1764, 1936, 2116, 2304, 2500, 2704, 2916, 3136, 3364, 3600, 3844, 4096,
            4356, 4624, 4900, 5184, 5476, 5776, 6084, 6400, 6724, 7056, 7396, 7744, 8100, 8464,
            8836, 9216, 9604, 10000,
        ];
        assert_eq!(v1, first_fifty_even_square());
    } */
    pub fn first_fifty_even_square() -> Vec<i32> {
        (1..)
            .filter(|&x| x % 2 == 0)
            .map(|x| x * x)
            .take(50)
            .collect()
    }