/* generics
Instructions
Create a function named identity which calculates the identity of a value (receives any data type and returns the same value).

Expected Function (signature to be completed)
pub fn identity(v: _) -> _ {
}
 */


pub fn identity<T>(v: T) -> T {
    v
}