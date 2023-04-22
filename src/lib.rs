#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use std::mem;

#[derive(Debug)]
pub struct LinkedList {
    // Some = non-empty list, None = empty list
    head: Option<Box<Node>>,
    len: usize,
}

#[derive(Debug)]
struct Node {
    // Some = point on existing Node, None = pointer not exist
    next: Option<Box<Node>>,
    elem: i32,
}

impl Node {
    fn new(elem: i32, next: Option<Box<Node>>) -> Node {
        Node {
            elem: elem,
            next: next,
        }
    }
}

impl LinkedList {
    fn check_index(&self, index: usize, include_border: bool) {
        if index > self.len + (usize::from(include_border) - 1) {
            panic!("Index {index} is out of bound! List length: {}", self.len)
        }
    }

    fn find_node(&mut self, index: usize) -> &mut Node {
        let mut curr_node: &mut Node = self
            .head
            .as_mut()
            .expect("BUG: check_index must have validated self.head exist");

        let count: usize = 0;

        while count < index {
            curr_node = curr_node
                .next
                .as_mut()
                .expect("BUG: Node under index must exist");
        }

        return curr_node;
    }

    pub fn new() -> LinkedList {
        LinkedList { head: None, len: 0 }
    }

    // fn push(&mut self, elem: i32) {

    // }

    pub fn insert(&mut self, index: usize, elem: i32) {
        self.check_index(index, true);

        if let None = self.head {
            self.head = Some(Box::new(Node::new(elem, None)))
        } else if index == 0 {
            self.head = Some(Box::new(Node::new(elem, mem::take(&mut self.head))));
        } else {
            let prev_node: &mut Node = self.find_node(index - 1);
            prev_node.next = Some(Box::new(Node::new(elem, mem::take(&mut prev_node.next))))
        }

        self.len += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_into_empty_success() {
        let mut list = LinkedList::new();

        list.insert(0, 99);

        let expected = r#"LinkedList {
    head: Some(
        Node {
            next: None,
            elem: 9,
        },
    ),
    len: 1,
}"#;
        assert_eq!(format!("{list:#?}"), expected);
    }
}
