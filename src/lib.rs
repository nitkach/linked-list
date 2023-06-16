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

impl FromIterator<i32> for LinkedList {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut list = LinkedList::new();

        let mut curr = &mut list.head;
        for val in iter {
            list.len += 1;

            curr = &mut curr.insert(Box::new(Node::new(val, None))).next;
            // *curr = Some(Box::new(Node::new(val, None)));

            // curr = &mut Option::insert(curr).next;
            // curr = &mut curr.as_mut().unwrap().next;
        }
        // let mut iter = iter.into_iter();


        // for val in iter {
        //     list.push(val);
        // }

        // while let Some(value) = iter.next() {
        //     list.push(value);
        // }

        list

    }
}

impl LinkedList {
    #[track_caller]
    fn check_index(&self, index: usize, include_border: bool) {
        assert!(
            index < self.len + usize::from(include_border),
            "Index {index} is out of bound! List length: {}",
            self.len
        );
    }

    fn create_new_linked_node(elem: i32, next: &mut Option<Box<Node>>) {
        *next = Some(Box::new(Node::new(elem, mem::take(next))));
    }

    fn find_node(&mut self, index: usize) -> &mut Node {
        let mut curr: &mut Node = self
            .head
            .as_mut()
            .expect("BUG: check_index must have validated self.head exist");

        for _ in 0..index {
            curr = curr
                .next
                .as_mut()
                .expect("BUG: Node under index must exist");
        }

        curr
    }

    #[must_use]
    pub const fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn push(&mut self, elem: i32) {
        self.insert(self.len, elem);
    }

    #[track_caller]
    pub fn insert(&mut self, index: usize, elem: i32) {
        self.check_index(index, true);

        // if self.head.is_none() {
        //     self.head = Some(Box::new(Node::new(elem, None)));
        if index == 0 {
            Self::create_new_linked_node(elem, &mut self.head);
        } else {
            let prev: &mut Node = self.find_node(index - 1);
            Self::create_new_linked_node(elem, &mut prev.next);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> i32 {
        Self::check_index(self, index, false);

        if index == 0 {
            // [LinkedList { head }] -> [10] -> [20] -> [30] -> None
            //
            let temp = mem::take(&mut self.head);
            //
            // temp = [LinkedList { head }] -> [10] -> [20] -> [30] -> None

            //
            //
            //
            let result = Option::unwrap(temp);

            self.head = result.next;

            return result.elem;
        }
        let prev = Self::find_node(self, index - 1);

        // [LinkedList { head }] -> [10]
        //
        //                          prev    next
        // [LinkedList { head }] -> [10] -> [20] -> [30] -> None
        //
        // Similar to:
        // let target_node = previous_node.next;
        // previous_node.next = None;
        let target = Option::take(&mut prev.next);

        let target = Option::unwrap(target);
        //                          prev
        // [LinkedList { head }] -> [10] -> None
        //
        // target_node
        // [20] -> [30] -> None
        let next = target.next;
        //                          prev
        // [LinkedList { head }] -> [10] -> None
        //
        // next_node
        // [30] -> None
        //
        // target_node
        // [20] -> uninitialised
        //
        //                     prev.next -> next_node
        // [LinkedList { head }] -> [10] -> [30] -> None
        prev.next = next;

        target.elem
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        let mut curr = &self.head;
        let mut count = 0;

        while let Some(curr_local) = curr {
            if count == index {
                break;
            }

            count += 1;

            curr = &curr_local.next;
        }

        // while !curr_node.is_none() && (count < index) {
        //     count += 1;
        //     curr_node = &curr_node.as_ref().unwrap().next;
        // };

        Some(curr.as_ref()?.elem)

        // match curr_node {
        //     Some(curr_node) => Some(curr_node.elem),
        //     None => None,
        // }

        // if !curr_node.is_none() {
        //     Some(curr_node.as_ref().unwrap().elem)
        // } else {
        //     None
        // }
    }

    pub fn set(&mut self, index: usize, value: i32) -> i32 {
        self.check_index(index, false);

        let target = self.find_node(index);

        let temp = target.elem;

        target.elem = value;

        temp
    }
}

#[cfg(test)]
mod tests;
