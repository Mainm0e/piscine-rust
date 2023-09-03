// todo : ref_cell
/* 
Instructions
Second part (lib.rs)
Now that you've created messenger, you can now create the following:

Create the Worker structure with the following fields:

track_value: which is the value that will be tracked by the tracker.

mapped_messages: that will store the latest messages from the Logger trait functions. This will be a HashMap. 
The key will represent the type of message (info, error or warning), and the value will be the actual message.

all_messages: that will be a vector of all messages sent.
Create the following associated functions for Worker:

new: that initializes a Worker structure.
Logger: to use the trait Logger, you must implement it for the Worker structure. Each function (warning, error and info) must insert the message to the respective field of the Worker structure.
You must use interior mutability, this means it must be possible to mutate data, even when there are immutable references to that data. Consequently, the user will not need to use the keyword mut. tip: RefCell. */


// todo : ref_cell
// Path: src/messenger.rs
// Compare this snippet from src/main.rs:
// use ref_cell::*;
//

mod messenger;
use std::collections::HashMap;
pub use crate::messenger::*;
pub use std::cell::RefCell;



pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}


impl Worker {
    pub fn new(value : usize) -> Worker{
        Worker { 
            track_value: Rc::new(value), 
            mapped_messages: RefCell::new(HashMap::new()), 
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {

    fn warning(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg[9..].to_string());
    }

    fn error(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg[7..].to_string());
    }

    fn info(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg[6..].to_string());
    }

    
}
 
/*  // !! Jay version
 mod messenger;
use std::collections::HashMap;
pub use crate::messenger::*;
pub use std::cell::RefCell;

pub struct Worker{
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: usize) -> Worker {
        Worker { 
            track_value: Rc::new(value), 
            mapped_messages: RefCell::new(HashMap::new()), 
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg[9..].to_string());
    }

    fn error(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg[7..].to_string());
    }

    fn info(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg[6..].to_string());
    }
} */