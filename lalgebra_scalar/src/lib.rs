//todo:lalgebra_scalar
/* 
Instructions
Define a Scalar trait which implements the operations Add, Sub, Mul, Div and any other restrictions you may need. Use a trait inheritance (supertraits)

Another condition for a number to be a scalar is to have a zero (as the neutral element in the addition), and a one (as the neutral element in the multiplication). Therefore the Scalar trait will require these two functions (described below).

After finishing completing the declaration of the trait, implement the Scalar trait for u32, u64, i32, i64, f32 and f64.

Expected Function (The signature must be completed)
You need add the impl for each cases asked in the subject

pub trait Scalar: _ {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
} */


/* //!! my copilot 
pub trait Scalar: Sized + Copy + std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> {
    type Item: std::ops::Add<Self, Output = Self>;
    
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i32 {
    type Item = i32;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for f32 {
    type Item = f32;
    
    fn zero() -> Self::Item {
        0.0
    }
    
    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f64 {
    type Item = f64;
    
    fn zero() -> Self::Item {
        0.0
    }
    
    fn one() -> Self::Item {
        1.0
    }
}
 */
use std::ops::{Add, Sub, Mul, Div};

/* // steve job
The Scalar trait uses trait inheritance to include Add, Sub, Mul,
and Div. Sized is also included to restrict types to those that 
have a known size at compile time.
*/
pub trait Scalar: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized {
    // Method to return the "zero" of the type
    fn zero() -> Self;
    
    // Method to return the "one" of the type
    fn one() -> Self;
}

// Implementing Scalar for u32
impl Scalar for u32 {
    fn zero() -> Self {
        0 // Zero for u32
    }

    fn one() -> Self {
        1 // One for u32
    }
}

// Implementing Scalar for u64
impl Scalar for u64 {
    fn zero() -> Self {
        0 // Zero for u64
    }

    fn one() -> Self {
        1 // One for u64
    }
}

// Implementing Scalar for i32
impl Scalar for i32 {
    fn zero() -> Self {
        0 // Zero for i32
    }

    fn one() -> Self {
        1 // One for i32
    }
}

// Implementing Scalar for i64
impl Scalar for i64 {
    fn zero() -> Self {
        0 // Zero for i64
    }

    fn one() -> Self {
        1 // One for i64
    }
}

// Implementing Scalar for f32
impl Scalar for f32 {
    fn zero() -> Self {
        0.0 // Zero for f32
    }

    fn one() -> Self {
        1.0 // One for f32
    }
}

// Implementing Scalar for f64
impl Scalar for f64 {
    fn zero() -> Self {
        0.0 // Zero for f64
    }

    fn one() -> Self {
        1.0 // One for f64
    }
}