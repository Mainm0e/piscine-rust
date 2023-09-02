// todo: drop_the_thread
/* 
Instructions
Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.

In this exercise, you will create a Drop Checker API.

Define the following structures:

Workers: containing:
drops: that will save the number of dropped threads.
states: that will save the state of multiple threads. If the thread is not dropped, the state will be false, and will be true otherwise.
Thread: containing:
pid: the id of the thread.
cmd: the name of the thread.
parent: a link to the structure Workers. (Tip: this should be a reference).
You'll need to also add the following associated functions to the structures:

Workers :

new: that creates a default worker.
new_worker: that returns a tuple with the pid and a new Thread. This function must receive a String representing the cmd.
is_dropped: that receives a pid and returns a bool that indicates the state of the thread.
track_worker: which returns a usize representing the length of the states vector. (The index of the next new thread).
add_drop: which is called by the Drop trait. It will receive a pid that will be used to change the state of the thread. If the state of that thread is true then it will panic with the message "X is already dropped", where X represents the pid). Otherwise it should change the state to true and increment the drops field by 1.
Thread:

new_thread: that initializes a new thread.
skill: that drops the thread.
You must implement the Drop trait for the Thread structure. In this trait you must call the function add_drop so that the state of the thread changes.

Expected Functions
use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {}
    pub fn new_worker(&self, c: String) -> (usize, Thread) {}
    pub fn track_worker(&self) -> usize {}
    pub fn is_dropped(&self, id: usize) -> bool {}
    pub fn add_drop(&self, id: usize) {}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    // expected public fields
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {}
    pub fn skill(self) {}
} */

use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new())
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let pid = self.track_worker();
        let thread = Thread::new_thread(pid, c, self);
        (pid, thread)
    }
    pub fn track_worker(&self) -> usize {
        let mut states = self.states.borrow_mut();
        states.push(false);
        states.len() - 1
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        let states = self.states.borrow();
        states[id]
    }
    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        } else {
            states[id] = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    cmd: String,
    parent: &'a Workers
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread {
            pid: p,
            cmd: c,
            parent: t
        }
    }
    pub fn skill(self) {
        self.parent.add_drop(self.pid);
    }
} 