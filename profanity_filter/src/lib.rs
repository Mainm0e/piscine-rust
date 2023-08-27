// todo: profanity filter
/* 
Instructions
Sometimes it is more desirable to catch the failure of some parts of a program instead of just calling panic.

For this exercise you will have to create a message blocker, where you must block the word stupid.

You will have to create a structure called Message, which contains:

elements:
content: String
user: String
associated functions:
new: which initializes the structure.
send_ms: which only has its implementation type (self) as argument. It should return None if the content of the message is either empty or contains the word stupid. It should return the content of the message otherwise.
You will also need to create a function named check_ms which accepts a reference to a Message, and returns a tuple. This function will invoke the send_ms function.

If send_ms returns None, then your function should return false and "ERROR: illegal".
Else, your function should return true and the contents of the message sent.
Expected Function
pub struct Message {

}

impl Message {
  pub fn new(ms: String, u: String) -> Message {

  }
  pub fn send_ms(&self) -> Option<&str> {

  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {

}
 */

 pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(s) => (true, s),
        None => (false, "ERROR: illegal"),
    }
}