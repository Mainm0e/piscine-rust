/* error types
Instructions
For this exercise, you will have to implement an error type for a form validator. This must validate the password and the first name.

The first name must not be empty and the password must have at least 8 characters, and a combination of alphabetic, numeric and none-alphanumeric (<, &, / ...).

Examples:

"asDd123=%": good.
"asgfD": error as it only contains alphabetic characters.
"asdsdf2": error as it is missing none-alphanumeric characters.
"sad\_#$": error as it is missing numeric characters.
Create a structure named Form that will have the following fields:

first_name: String
last_name: String
birth: NaiveDate that will convert a string "2015-09-05" to a date of that format.
birth_location: String
password: String
You must implement the associated functions new and validate that will validate the form.

For the error type you must create a struct named FormError. It must have the fields:

form_values: this will be a tuple of strings representing the invalid input. For example: ("password", "asdaSD\_") or ("first_name", "someone")

date: that will have the date that the error occurred in the format "2020-12-14 09:33:41"

err: the error description:

"No user name"
"At least 8 characters"
"Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"
Dependencies
chrono = "0.4"

Expected Function
pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {}
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {}
    
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {}
} */
pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
   pub form_values: (String, String),
   pub date: String,
   pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form{
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut errors: Vec<&str> = Vec::new();
        if self.first_name.is_empty() {
            errors.push("No user name");
        }
        if self.password.len() < 8 {
            errors.push("At least 8 characters");
        }
        
        let mut has_alphabetic = false;
        let mut has_numeric = false;
        let mut has_none_alphanumeric = false;
        
        for c in self.password.chars() {
            if c.is_alphabetic() {
                has_alphabetic = true;
            } else if c.is_numeric() {
                has_numeric = true;
            } else if !c.is_whitespace() {
                has_none_alphanumeric = true;
            }
        }
        if self.password.len() >= 8 {
            if !(has_alphabetic && has_numeric && has_none_alphanumeric) {
                errors.push("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)");
            }
        }
        
        if errors.is_empty() {
            Ok(vec!["Valid first name", "Valid password"])
        } else {
            Err(FormError::new(
                if self.first_name.is_empty() {
                    String::from("first_name")
                } else {
                    String::from("password")
                },
                if self.first_name.is_empty() {
                    self.first_name.clone()
                } else {
                    self.password.clone()
                },
                errors.join(", "),
            ))
        }
    }

}
    
