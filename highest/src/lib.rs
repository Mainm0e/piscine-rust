/* Highest
Instructions
In this exercise, a Numbers struct will be given.

These methods have to be written for it:

new: create a new instance of Numbers.
List: which returns an array with every number in the struct.
Latest: which returns an Option<u32> with the last added number.
Highest: which returns an Option<u32> with the highest number from the list.
Highest_Three: which returns a Vec<u32> with the three highest numbers.
Expected functions
#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl Numbers {
    pub fn new(numbers: &[u32]) -> Self {}

    pub fn list(&self) -> &[u32] {}

    pub fn latest(&self) -> Option<u32> {}

    pub fn highest(&self) -> Option<u32> {}

    pub fn highest_three(&self) -> Vec<u32> {}
}
Usage
Here is a program to test your function.

use highest::*;

fn main() {
    let expected = [30, 500, 20, 70];
    let n = Numbers::new(&expected);
    println!("{:?}", n.list());
    println!("{:?}", n.highest());
    println!("{:?}", n.latest());
    println!("{:?}", n.highest_three());
}
And its output:

$ cargo run
[30, 500, 20, 70]
Some(500)
Some(70)
[500, 70, 30]
$
Notions
 */

 #[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &'a [u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.is_empty() {
            None
        } else {
            Some(self.numbers[self.numbers.len() - 1])
        }
    }

    pub fn highest(&self) -> Option<u32> {
        if self.numbers.is_empty() {
            None
        } else {
            Some(*self.numbers.iter().max().unwrap())
        }
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted_numbers = self.numbers.to_vec();
        sorted_numbers.sort_by(|a, b| b.cmp(a));
        sorted_numbers.truncate(3);
        sorted_numbers
    }
}
