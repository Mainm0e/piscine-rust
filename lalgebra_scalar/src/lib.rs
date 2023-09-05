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


pub trait Scalar: Sized + Copy + std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> {
    type Item;
    
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
