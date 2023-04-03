use std::ops::Range;

fn main() {}

struct LinkedList(*mut Node);

struct Node {
    first: u64,
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
                first: value,
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

            node.first + Self::sum(&node.rest)
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { Box::from_raw(self.0 as *mut Node) };
        }
    }
}

struct Iter<'a> {
    list: LinkedList,
    _marker: std::marker::PhantomData<&'a u64>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.0.is_null() {
            None
        } else {
            let mut node = unsafe { std::ptr::read(self.list.0) };

            std::mem::swap(&mut node.rest, &mut self.list);
            std::mem::forget(node.rest);

            Some(node.first)
        }
    }
}

impl LinkedList {
    fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        Iter {
            list: LinkedList(self.0),
            _marker: std::marker::PhantomData,
        }
    }

    fn reverse(&mut self) {
        let this = std::mem::take(self);
        *self = Self::reverse_help(LinkedList(std::ptr::null_mut()), this);
    }

    fn reverse_help(left: Self, right: Self) -> Self {
        if right.0.is_null() {
            left
        } else {
            let mut node = unsafe { std::ptr::read(right.0) };

            let rest = node.rest;
            node.rest = left;

            unsafe { std::ptr::write(right.0, node) };

            Self::reverse_help(right, rest)
        }
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
