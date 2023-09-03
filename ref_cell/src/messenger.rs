/* First part (messenger.rs)
Create a module named messenger. This module will be able to inform a user of how many references of a given value they are using. The main objective of this module is to limit how many times a value is referenced.

For this module the following must be created:

Implement Logger: a trait which implements the following three functions:

fn warning(&self, msg: &str);
fn info(&self, msg: &str);
fn error(&self, msg: &str);
Implement the Tracker structure with the following fields:

logger: a reference to Logger.
value: the count of how many times the value was referenced. It should not exceed max.
max: the max count of references.
Add the following associated functions to Tracker:

new: that initializes the structure.
set_value: that sets the value. It should compare the number of references to value and max to work out the percentage used. It should write to the following traits if it exceeds the specified usage percentage:
percentage >= 100%: "Error: you are over your quota!" should be written to error.
percentage >= 70% and percentage < 100%: "Warning: you have used up over X% of your quota! Proceeds with precaution" should be written to warning, where X should be replaced with the calculated percentage.
peek: that will take a peek at how much usage the variable already has. It should write "Info: you are using up to X% of your quota" to the info trait function. X should be replaced with the calculated percentage. */


pub use std::rc::Rc;
    pub trait Logger {
        fn warning(&self, msg: &str);
        fn info(&self, msg: &str);
        fn error(&self, msg: &str);
    }

    pub struct Tracker<'a, T: Logger> {
       pub  logger: &'a T,
       pub  value: Rc<usize>,
       pub  max: usize,
    }

    impl<'a, T: Logger> Tracker<'a, T> {
        pub fn new(logger: &'a T , max: usize) -> Tracker<'a, T> {
            Tracker {
                logger,
                value: Rc::new(0),
                max,
            }
        }

        pub fn set_value(&mut self, value: &'a Rc<usize>) {
            let percentage: f64 = Rc::strong_count(value) as f64 / self.max as f64;

            if percentage >= 1.0{
                self.logger.error("Error: you are over your quota!");
            }else if percentage >= 0.7 && percentage < 1.0{
                let message = format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", (percentage * 100.0) as i64);
                self.logger.warning(&message);
            }
        }

        pub fn peek(&self,value: &'a Rc<usize>) {
            let percentage: f64 = Rc::strong_count(value) as f64 / self.max as f64;
            let message = format!("Info: you are using up to {}% of your quota", (percentage * 100.0) as i64);
            self.logger.info(&message);
        }

    
    } 


/* // !! Jay version 
    pub use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger>{
    pub logger: &'a T,
    pub value: Rc<usize>,
    pub max: usize,
}

impl<'a, T> Tracker<'a, T>
where T: Logger {
    pub fn new(logger: &'a T, max: usize) -> Tracker<'a, T>{
        Tracker {
            logger,
            value: Rc::new(0),
            max,
        }
    }

    pub fn set_value(&'a self, value: &'a Rc<usize>) {
        let percentage: f64 = Rc::strong_count(value) as f64 / self.max as f64;

        if percentage >= 1.0{
            self.logger.error("Error: you are over your quota!");
        }else if percentage >= 0.7 && percentage < 1.0{
            let message = format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", (percentage * 100.0) as i64);
            self.logger.warning(&message);
        }
    }

    pub fn peek(&self, track_value: &'a Rc<usize>) {
        let percentage: f64 = Rc::strong_count(track_value) as f64 / self.max as f64;
        let message = format!("Info: you are using up to {}% of your quota", (percentage * 100.0) as i64);
        self.logger.info(&message);
    }
} */