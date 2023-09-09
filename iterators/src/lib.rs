/* iterators
Instructions
The Collatz Conjecture or 3x+1 problem can be summarized as follows:

Take any positive integer n.

If n is even, you will divide n by 2 to get n / 2.
If n is odd, you will multiply n by 3 and add 1 to get 3n + 1.
Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.

But sometimes the number grow significantly before it reaches 1. This can lead to an integer overflow and makes the algorithm unsolvable within the range of a number in u64. You will not have to worry about that in this exercise.

Given a number n, return the number of steps required to reach 1.

Examples:

Starting with n = 16, the steps would be as follows:

16
8
4
2
1
Resulting in 4 steps. So for input n = 16, the return value would be 4.

Notions
Trait Iterator
Collatz Conjecture
Expected functions
#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {}

impl Collatz {
	pub fn new(n: u64) -> Self {}
}

pub fn collatz(n: u64) -> usize {}
Usage
Here is a program to test your function.

use iterators::*;

fn main() {
    println!("{:?}", collatz(0));
    println!("{:?}", collatz(1));
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(12));
}
And its output:

$ cargo run
0
0
2
5
8
16
9
$ */

/* 
#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            None
        } else if self.v % 2 == 0 {
            self.v /= 2;
            Some(self.v)
        } else {
            self.v = self.v * 3 + 1;
            Some(self.v)
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}


pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}

 */

 #[derive(Copy, Clone)]
pub struct Collatz {
    v: u64,
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            None
        } else if self.v % 2 == 0 {
            self.v = self.v / 2; // Update self.v here
            Some(self.v * 2)    // Return the previous self.v
        } else {
            self.v = self.v * 3 + 1; // Update self.v here
            Some(self.v / 3)        // Return the previous self.v
        }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}
