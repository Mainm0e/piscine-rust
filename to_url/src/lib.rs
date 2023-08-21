// to_url
/* 
## Instructions
Create a function named to_url which takes a string and substitutes every white-space with "%20".

Expected functions
pub fn to_url(s: &str) -> String {
}
 */

pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}