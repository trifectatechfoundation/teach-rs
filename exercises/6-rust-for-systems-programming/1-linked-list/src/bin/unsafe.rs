// In this exercise we'll experience some of the pain of unsafe rust. it's just less nice than
// "normal", safe rust. But with great responsibility comes great power.
//
// We'll implement various functions for linked lists, and an iterator over linked lists
// find and fix the TODOs, to make the tests run and pass.
//
// > cargo test -p F1-linked-list
//
// It is quite likely that you will run into SEGFAULTs or similar problems in this exercise. Please
// let us know on the discord if you get stuck!
use std::ops::Range;

fn main() {}

struct LinkedList(*mut Node);

struct Node {
    current: u64,
    rest: LinkedList,
}

impl Default for LinkedList {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        let mut this = LinkedList(std::ptr::null_mut());
        for value in range.rev() {
            let node = Node {
                current: value,
                rest: this,
            };

            this = LinkedList(Box::into_raw(Box::new(node)));
        }

        this
    }

    fn sum(&self) -> u64 {
        if self.0.is_null() {
            0
        } else {
            let node = unsafe { std::ptr::read(self.0) };

            node.current + Self::sum(&node.rest)
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        // TODO: implement drop
    }
}

struct Iter<'a> {
    list: *const Node,
    _marker: std::marker::PhantomData<&'a u64>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement the next function
        //
        // make sure that `Node` values are never dropped here! An implementation is possible
        // without any of the `std::ptr` functions, just dereferencing is sufficient.
        None
    }
}

impl LinkedList {
    fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        Iter {
            list: self.0,
            _marker: std::marker::PhantomData,
        }
    }

    fn reverse(&mut self) {
        // TODO: reverse the linked list in-place. The general approach is to start with a new empty
        // linked list, and move elements from self over to this new list. Finally update self with
        // the new list.
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn test_iter() {
        let list = LinkedList::range(0..5);

        assert_eq!(vec![0, 1, 2, 3, 4], list.iter().collect::<Vec<_>>())
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::range(0..5);

        list.reverse();

        assert_eq!(vec![4, 3, 2, 1, 0], list.iter().collect::<Vec<_>>())
    }
}
