#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use std::mem;

#[derive(Debug)]
pub struct LinkedList {
    // Some = non-empty list, None = empty list
    head: Option<Box<Node>>, // Box<Option<Node>>
    len: usize,
}

#[derive(Debug)]
struct Node {
    // Some = point on existing Node, None = pointer not exist
    next: Option<Box<Node>>,
    #[allow(dead_code)]
    elem: i32,
}

impl Node {
    const fn new(elem: i32, next: Option<Box<Self>>) -> Self {
        Self { next, elem }
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkedList {
    #[track_caller]
    fn check_index(&self, index: usize, include_border: bool) {
        assert!(
            index <= self.len + (usize::from(include_border) - 1),
            "Index {index} is out of bound! List length: {}",
            self.len
        );
    }

    fn find_node(&mut self, index: usize) -> &mut Node {
        let mut curr_node: &mut Node = self
            .head
            .as_mut()
            .expect("BUG: check_index must have validated self.head exist");

        for _ in 0..index {
            curr_node = curr_node
                .next
                .as_mut()
                .expect("BUG: Node under index must exist");
        }

        curr_node
    }

    #[must_use]
    pub const fn new() -> Self {
        Self { head: None, len: 0 }
    }

    // fn push(&mut self, elem: i32) {

    // }

    fn create_node(elem: i32, next: &mut Option<Box<Node>>) -> Option<Box<Node>> {
        Some(Box::new(Node::new(elem, mem::take(next))))
    }

    #[track_caller]
    pub fn insert(&mut self, index: usize, elem: i32) {
        self.check_index(index, true);

        // if self.head.is_none() {
        //     self.head = Some(Box::new(Node::new(elem, None)));
        if index == 0 {
            self.head = Self::create_node(elem, &mut self.head);
        } else {
            let prev_node: &mut Node = self.find_node(index - 1);
            prev_node.next = Self::create_node(elem, &mut prev_node.next);
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

        list.insert(1, 99);

        let expected = r#"LinkedList {
    head: Some(
        Node {
            next: None,
            elem: 99,
        },
    ),
    len: 1,
}"#;
        assert_eq!(format!("{list:#?}"), expected);
    }
}
