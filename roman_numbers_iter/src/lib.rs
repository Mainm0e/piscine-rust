/* roman_numbers_iter
Instructions
Implement the Iterator trait for the RomanNumber type. You should use the code from the previous exercise roman_numbers.

Notions
Trait Iterator
Expected Functions
//...

impl Iterator for RomanNumber {}
Usage
Here is a program to test your function.

use roman_numbers_iterator::RomanNumber;

fn main() {
	let mut number = RomanNumber::from(15);

	println!("{:?}", number);
	println!("{:?}", number.next());
}
And its output

$ cargo run
RomanNumber([X, V])
Some(RomanNumber([X, V, I]))
$ */

use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        println!("value: {}", value);
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Invalid value"),
        }
    }

}


impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        let mut result = Vec::new();
        let mut value = value;
       /*  if value == 0 {
            result.push(Nulla);
        } */
        while value > 0 {
            let mut digit = 1;
            while digit * 10 <= value {
                digit *= 10;
            }
            let mut count = value / digit;
            if count == 9 {
                result.push(digit.into());
                result.push((digit * 10).into());
                count = 0;
            } else if count >= 5 {
                result.push((digit * 5).into());
                count -= 5;
            } else if count == 4 {
                result.push(digit.into());
                result.push((digit * 5).into());
                count = 0;
            }
            for _ in 0..count {
                result.push(digit.into());
            }
            value %= digit;
        }
        RomanNumber(result)
    }
}
impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.0.clone(); // Clone the vector
        let mut value = 0;

        // Calculate the decimal value of the current Roman numeral
        for digit in result.iter() {
            value += match digit {
                Nulla => 0,
                I => 1,
                V => 5,
                X => 10,
                L => 50,
                C => 100,
                D => 500,
                M => 1000,
            };
        }

        value += 1;

        if value > 3999 {
            return None;
        }

        if value == 0 {
            result.push(Nulla);
        }

        while value > 0 {
            let mut digit = 1;
            while digit * 10 <= value {
                digit *= 10;
            }

            let mut count = value / digit;

            if count == 9 {
                result.push(digit.into());
                result.push((digit * 10).into());
                count = 0;
            } else if count >= 5 {
                result.push((digit * 5).into());
                count -= 5;
            } else if count == 4 {
                result.push(digit.into());
                result.push((digit * 5).into());
                count = 0;
            }

            for _ in 0..count {
                result.push(digit.into());
            }

            value %= digit;
        }

        // Update self.0 with the new result vector
        self.0 = result.clone();

        Some(RomanNumber(result))
    }
}
