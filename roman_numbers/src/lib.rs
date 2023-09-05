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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
    Nulla,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RomanNumber(Vec<RomanDigit>);

impl RomanNumber {
    fn from_u32(num: u32) -> RomanNumber {
        let mut num = num;
        let mut result = Vec::new();
        
        // Define the Roman numeral digits and their values.
        let digits = [
            (1000, RomanDigit::M),
            (900, RomanDigit::C),
            (500, RomanDigit::D),
            (400, RomanDigit::C),
            (100, RomanDigit::C),
            (90, RomanDigit::X),
            (50, RomanDigit::L),
            (40, RomanDigit::X),
            (10, RomanDigit::X),
            (9, RomanDigit::I),
            (5, RomanDigit::V),
            (4, RomanDigit::I),
            (1, RomanDigit::I),
            (0, RomanDigit::Nulla)
        ];

        // Iterate through the digits and their values.
        for (value, digit) in digits.iter() {
            // While the value is less than the number, add the digit to the result.
            while *value <= num {
                result.push(*digit);
                num -= *value;
            }
        }

        RomanNumber(result)
    }
}

impl From<u32> for RomanNumber {
    fn from(num: u32) -> Self {
        RomanNumber::from_u32(num)
    }
}


