/* generics_list
Instructions
Create a linked list of generic values with the following methods.

new: returns a new empty list.
push: adds an element to the beginning of the list.
pop: deletes an element from the list based on LIFO.
len: returns the size of the list.
Expected Functions
#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
    }

    pub fn push(&mut self, value: T) {
    }

    pub fn pop(&mut self) {
    }

    pub fn len(&self) -> usize {
    }
} */

#[derive(Clone, Debug)] 
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)] 
pub struct Node<T> {

    // T is a generic type
    pub value: T,

    // Box is a smart pointer that points to a heap allocated value of type T
    pub next: Option<Box<Node<T>>>,
}


impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value: value,
            next: self.head.take().map(Box::new),
        };

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        let old_head = self.head.take();
        self.head = old_head.and_then(|node| node.next.map(|node| *node));
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut aux = self.head.as_ref();

        while let Some(node) = aux {
            count += 1;
            aux = node.next.as_ref().map(|node| &**node);
        }

        count
    }
}

