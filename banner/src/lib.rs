// todo: banner
/* 
Instructions
"Result is a better version of the Option type that describes a possible error instead of possible absence".

Create a structure called Flag which has the following elements:

short_hand: String
long_hand: String
desc: String
This structure must have a function called opt_flag which initializes the structure. 
This function receives two string references and returns a structure Flag. Here is an example of its usage:

    let d = Flag::opt_flag("diff", "gives the difference between two numbers");

    println!("short hand: {}, long hand: {}, description: {}", d.short_hand, d.long_hand, d.desc);
    // output: "short hand: -d, long hand: --diff, description: gives the difference between two numbers"
A second structure named FlagsHandler will be given which just has one element: flags: HashMap<(String, String), Callback>. 
You'll need to implement the following associated functions" (methods) associated with FlagsHandler are for you to complete:"

add_flag, which adds the flag and callback function to the HashMap.
exec_func, which executes the function using the flag provided and returns the result. The result will be either a string with the value from the callback or an error.
A type called Callback will also be provided. It is a function which is going to be used in the structure and functions above. 
This function will be the callback for the flag associated to it.

You will have to create the following callback functions:

div: which converts the reference strings to f32s and returns the Result, as the division of the f32s or the standard (std) error: ParseFloatError.
rem: which converts the reference strings to f32s and returns the Result, as the remainder of the division of the f32s or the standard (std) error ParseFloatError.
Expected Function
use std::collections::HashMap;

pub struct Flag {
    // expected public fields
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {

    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {

    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {

    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {

}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {

} */
use std::collections::HashMap;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str)->Flag{
        Flag{
            short_hand: "-".to_string() + &l_h.chars().nth(0).unwrap().to_string(),
            long_hand: "--".to_string() + l_h,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, std::num::ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        let func = self.flags.get(&flag).unwrap();
        let result = func(argv[0], argv[1]);
        match result {
            Ok(s) => s,
            Err(e) => e.to_string(),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, std::num::ParseFloatError> {
    let a = a.parse::<f32>()?;
    let b = b.parse::<f32>()?;
    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, std::num::ParseFloatError> {
    let a = a.parse::<f32>()?;
    let b = b.parse::<f32>()?;
    Ok((a % b).to_string())
}