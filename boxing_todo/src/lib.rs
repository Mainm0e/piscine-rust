/* boxing_todo
Instructions
The objective is to create an API to parse a list of todos that are organized in a JSON file. You must handle all possible errors in a multiple error system.

Organization of the JSON file:

{
  "title": "TODO LIST FOR PISCINE RUST",
  "tasks": [
    { "id": 0, "description": "do this", "level": 0 },
    { "id": 1, "description": "do that", "level": 5 }
  ]
}
err.rs
Create a module in a file named err.rs which handles the boxing of errors.

This module must implement an enum called ParseErr which will take care of the parsing errors. It must have the following elements:

Empty
Malformed: which has a dynamic boxed error as element
A structure called ReadErr which will take care of the reading errors, with an element called child_err of type Box<dyn Error>.

For each data structure, you will have to implement a function called fmt for the Display trait. It should write the message "Fail to parse todo" in the case of any parsing error. Otherwise, it should write the message "Fail to read todo file".

For the Error trait, the following functions (methods) have to be implemented:

source which returns an Option with the error:

For the ReadErr, it must return the option with the error.
For the ParseErr, it will return an option which is None if the tasks are empty, and the error if the parsing is malformed.
lib.rs
In the lib file you will have to implement a function called get_todo which receives a string and returns a Result which can be the structure TodoList or a boxing error. This function must be able to deserialize the json file.

Basically it must parse and read the JSON file and return the TodoList if everything is fine, otherwise it returns the error.

for lib.rs

mod err;
use err::{ ParseErr, ReadErr };

pub use json::{parse, stringify};
pub use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {

    }
} */

mod err;
use err::{ ParseErr, ReadErr };

pub use json::{parse, stringify};
pub use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file = std::fs::read_to_string(path)
        .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }))?;
    
    if file.is_empty() {
        return Err(Box::new(ReadErr { child_err: Box::new(ParseErr::Empty) }));
    }   
        println!("1{:?}", file);
        // if file.is_empty() expected substring: `"Fail to parses todo Some(Malformed(UnexpectedCharacter { ch: ',', line: 1, column: 15 }))"`
        let json = parse(&file)
        .map_err(|e| Box::new(ParseErr::Malformed(Box::new(ParseErr::Malformed(Box::new(e))))))?;

        println!("2{:?}", json);
        let mut tasks: Vec<Task> = Vec::new();
        let mut title: String = String::new();
        if json["tasks"].is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }
        for (key, value) in json.entries() {
            if key == "title" {
                title = value.to_string();

            } else if key == "tasks" {
                for task in value.members() {
                    let id = task["id"].as_u32().unwrap();
                    let description = task["description"].to_string();
                    let level = task["level"].as_u32().unwrap();
                    tasks.push(Task { id, description, level });
                }
            }
        }
        Ok(TodoList { title, tasks })

    }

}



