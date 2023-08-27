// todo: unwrap_or_expect
/* 
Instructions
Create a function named fetch_data which has two arguments:

server: Which is a Result having the server url or an error message inside.
security_level: Which is an enum defining the behavior of the function in case of errors.
The security_level will work as follow:

Unknown: The function panics without printing any custom message.
High: The function panics and prints the error message ERROR: program stops.
Medium: The function returns the string WARNING: check the server.
Low: The function returns the string Not found: [SERVER_URL].
BlockServer: The function will panic only if the Result value is Ok and the error message will be the string contained in Ok.
To return from fetch_data you must use expect, unwrap_or, unwrap_err, unwrap and unwrap_or_else.

Expected Functions
pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {

} */

pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.expect("called `Result::unwrap()` on an `Err` value"),
        Security::High => server.expect("ERROR: program stops"),
        Security::Medium => server.unwrap_or("WARNING: check the server".to_string()),
        Security::Low => server.unwrap_or_else(|err| format!("Not found: {}", err)),
        Security::BlockServer => server.unwrap_err(),
    }
}
