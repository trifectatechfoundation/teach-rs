use std::mem::ManuallyDrop;

// An implementation of the `Result` type using explicitly tagged unions
//
// fix the TODOs, and veryify that your implementation mirrors the one of `std::result::Result`
// using tests.

struct RocResult<T, E> {
    tag: RocResultTag,
    payload: RocResultUnion<T, E>,
}

enum RocResultTag {
    Ok,
    Err,
}

// HINT: read the documentation of ManuallyDrop, and see what methods it provides
union RocResultUnion<T, E> {
    ok: ManuallyDrop<T>,
    err: ManuallyDrop<E>,
}

impl<T, E> Drop for RocResult<T, E> {
    fn drop(&mut self) {
        todo!()
    }
}

impl<T, E> RocResult<T, E> {
    fn unwrap(mut self) -> T {
        match self.tag {
            RocResultTag::Ok => unsafe { ManuallyDrop::take(&mut self.payload.ok) },
            RocResultTag::Err => panic!("Called `unwrap` on an Err"),
        }
    }

    fn ok(v: T) -> Self {
        todo!()
    }

    fn err(e: E) -> Self {
        todo!()
    }

    fn map<F, U>(mut self, f: F) -> RocResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // TODO: test your implementations
}

fn main() {
    println!("Hello, world!");
}
