/* roman_numbers
Instructions
Implement the From<u32> trait to create a roman number from a number. The roman number should be in subtractive notation (the common way to write roman number I, II, III, IV, V, VI, VII, VIII, IX, X ...).

Start by defining the digits as RomanDigit (Nulla is 0).

Next define RomanNumber as a wrapper to a vector of RomanDigit, and implement the Trait From<u32>.

Expected Functions and Data Structures
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

#[derive(Debug, Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
}

impl From<u32> for RomanNumber {
} */
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
        RomanNumber(result)
    }
}