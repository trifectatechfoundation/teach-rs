use std::ops::Range;

enum LinkedList {
    Nil,
    Cons(u64, Box<LinkedList>),
}

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        use LinkedList::*;

        let mut this = Nil;
        for value in range.rev() {
            this = Cons(value, Box::new(this));
        }

        this
    }

    fn sum(&self) -> u64 {
        use LinkedList::*;

        match self {
            Nil => 0,
            Cons(first, rest) => first + rest.sum(),
        }
    }
}

fn main() {
    let list = LinkedList::range(0..10);
    println!("sum: {}", list.sum());
}
