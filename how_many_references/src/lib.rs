// todo : how many references
/* 
Instructions
Create the following functions:

add_element: which adds an element to the list in the Node.
how_many_references: which returns how many times the value is referenced in the code.
rm_all_ref: which accepts an Rc<String> and removes all elements from the vector that are equal to that value. This should only happen if the two Rcs point to the same allocation.
Expected Functions and structures
pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {}
    pub fn rm_all_ref(&mut self, element: Rc<String>) {}
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {}
Usage */

pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|x| x != &element);
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}