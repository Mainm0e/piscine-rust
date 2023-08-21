//todo: fibonacci2
/* 
Instructions
Complete the body of the function fibonacci.

 pub fn fibonacci(n: u32) -> u32 {
 }
This function receives a number n and returns the nth number in the fibonacci series.

The Fibonacci Series starts like this: 0, 1, 1, 2, 3, 5, 8, 13 etc... */

pub fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    a
}