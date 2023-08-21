// todo:tuples_refs
/* tuples_refs
Instructions
Define a tuple struct to represent a Student. Each is identified by an id of type u32, their first name and last name.

Then define three functions to return the id, first name and last name.

pub fn id(student: &Student) -> u32 {
}

pub fn first_name(student: &Student) -> String {
}

pub fn last_name(student: &Student) -> String {
} */

#[derive(Debug)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.clone()
}

pub fn last_name(student: &Student) -> String {
    student.2.clone()
}