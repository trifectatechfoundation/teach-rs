/// One way to implement a queue is to use a linked list; however, that requires a lot of dynamic memory manipulation to add/remove individual items.
/// A more low-level approach is to use a circular buffer: the compromise is that the capacity of the queue is then "fixed". For a background on circular buffers,
/// you can consult https://en.wikipedia.org/wiki/Circular_buffer

// A partial implementation is provided below; please finish it and add some more methods; please remember to run 'cargo fmt' and 'cargo clippy' after
// every step to get feedback from the rust compiler!

// 1) implement read()

// 2) the queue now has a fixed size; change the definition so that the data member becomes a Box<[u8]>; you can use the provided function 'make_box' to make
// boxed slices of arbitrary sizes. Make changes to your method definitions as needed (the definition of 'write' should not need changes!)

// 3) change the method 'new()' into 'new(size: usize)' that initializes a ring buffer of the given size (instead of a fixed size of 16); use the 'make_box' function.

// 4) in a queue that has size N, how many elements can be stored at one time? (test your answer experimentally)

// 5) EXTRA EXERCISES:
//  - add a method "has_room" so that "queue.has_room()" is true if and only if writing to the queue will succeed
//  - add a method "peek" so that "queue.peek()" returns the same thing as "queue.read()", but leaves the element in the queue

struct RingBuffer {
    data: [u8; 16],
    start: usize,
    end: usize,
}

impl RingBuffer {
    fn new() -> RingBuffer {
        RingBuffer {
            data: [0; 16],
            start: 0,
            end: 0,
        }
    }

    /// This function tries to read a value from the queue and returns Some(value) if this succeeds,
    /// it returns None if the queue was empty

    fn read(&mut self) -> Option<u8> {
        todo!()
    }

    /// This function tries to put `value` on the queue; and returns true if this succeeds
    /// It returns false is writing to the queue failed (which can happen if there is not enough room)

    fn write(&mut self, value: u8) -> bool {
        self.data[self.end] = value;
        let pos = (self.end + 1) % self.data.len();
        if pos == self.start {
            // the buffer can hold no more new data
            false
        } else {
            self.end = pos;

            true
        }
    }
}

/// This function creates an "owned slice" a user-selectable size by allocating it as a vector (filled with zeores) using vec![], and then turning it
/// into a Box<[u8]> using the into_boxed_slice() method, see https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_boxed_slice

fn make_box(reqsize: usize) -> Box<[u8]> {
    vec![0; reqsize].into_boxed_slice()
}

/// This is a fun extra bit: by defining an "iterator", a ring buffer we defined ourselves can be used in for loops! (We will explain this feature in a later module!)

impl Iterator for RingBuffer {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.read()
    }
}

fn main() {
    let mut queue = RingBuffer::new();
    assert!(queue.write(1));
    assert!(queue.write(2));
    assert!(queue.write(3));
    assert!(queue.write(4));
    assert!(queue.write(5));
    for elem in queue {
        println!("{elem}");
    }
}
