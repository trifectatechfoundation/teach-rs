const STACK_BUF_SIZE: usize = 32;

/// A growable, generic list that resides on the stack if it's small,
/// but is moved to the heap to grow larger if needed.
pub enum LocalStorageVec {
    // TODO add some variants containing data
    // to make the compiler happy
}

/* Put your impl blocks here */

/* 
Below `From` implementations are used in the tests and are therefore given. However,
   you should have a thourough look at them as they contain various new concepts
*/
#[cfg(feature = "b")]
impl<T> From<&[T]> for LocalStorageVec<T>
where  
    T: Default + Copy,
{
    fn from(value: &[T]) -> Self {
        if value.len() <= STACK_BUF_SIZE {
            // Here, we're trying to crate an array `[T; STACK_BUF_SIZE]` 
            // The arrow brackets `<` and `>` are here for parsing reasons.
            match <[T; STACK_BUF_SIZE]>::try_from(value) {
                Ok(buf) => Self::Stack {
                    buf,
                    len: STACK_BUF_SIZE,
                },
                Err(_) => {
                    // Working with unitialized memory is deemed `unsafe`
                    // in Rust, and as such we'll instantiate the array with
                    // `T`'s default values, which is why `T` needs to implement
                    // `Default`. The array initialization syntax `[val; count]`
                    // only works for types that are `Copy`. Therefore, `T`
                    // needs to both implment `Default` and `Copy` for this `From`
                    // implementation to work. Once we're more proficient in working
                    // with uninitialized memory, we can lose the bounds.
                    let mut buf = [T::default(); STACK_BUF_SIZE];
                    // `copy_from_slice` copies all items from the passed slice to
                    // the slice it's called on. It `panic`s if those slices are not
                    // of the same length, so we ensure this is the case by indexing `buf`
                    buf[..value.len()].copy_from_slice(value);
                    Self::Stack {
                        buf,
                        len: value.len(),
                    }
                }
            }
        } else {
            // The items cannot fit in the stack-backed array, so we'll put
            // them on the heap
            let v = value.to_vec();
            Self::Heap(v)
        }
    }
}

#[cfg(feature = "disabled")]
impl<T: Default + Copy, const N: usize> From<&[T]> for LocalStorageVec<T, N> {
    fn from(value: &[T]) -> Self {
        if value.len() <= N {
            match <[T; N]>::try_from(&value[..]) {
                Ok(buf) => Self::Stack { buf, len: N },
                Err(_) => {
                    let mut buf = [T::default(); N];
                    buf[..value.len()].copy_from_slice(&value[..]);
                    Self::Stack {
                        buf,
                        len: value.len(),
                    }
                }
            }
        } else {
            let v = value.to_vec();
            Self::Heap(v)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{LocalStorageVec, STACK_BUF_SIZE};

    #[test]
    // Don't remove the #[ignore] attribute!
    #[ignore = "This test is just to validate the definition of `LocalStorageVec`"]
    #[allow(unreachable_code, unused_variables)]
    fn it_compiles() {
        // Here's a trick to 'initialize' a type while not actually
        // creating a value: an infinite `loop` expression diverges
        // and evaluates to the never type `!`, which, as is can never
        // actually be instantiated, coerces to any other type.
        // Some other ways of diverging are by calling the `panic!` or the `todo!`
        // macros.
        // More info:
        // - https://doc.rust-lang.org/rust-by-example/fn/diverging.html
        // - https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops
        let vec: LocalStorageVec<u32> = loop {};
        match vec {
            LocalStorageVec::Stack { buf, len } => {
                let _buf: [u32; STACK_BUF_SIZE] = buf;
                let _len: usize = len;
            }
            LocalStorageVec::Heap(v) => {
                let _v: Vec<u32> = v;
            }
        }
    }

    #[test]
    #[cfg(feature = "c")]
    fn it_pushes() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::new();
        for value in 0..128 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Stack { len: 128, .. }));
        for value in 128..256 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Heap(v) if v.len() == 256))
    }

    #[test]
    #[cfg(feature = "c")]
    fn it_lens() {
        let vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
        let vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    #[cfg(feature = "c")]
    fn it_pops() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        for _ in 0..128 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);
    }

    #[test]
    #[cfg(feature = "d")]
    fn it_inserts() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        vec.insert(1, 3);
        assert!(matches!(
            vec,
            LocalStorageVec::Stack {
                buf: [0, 3, 1, 2],
                len: 4
            }
        ));

        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3]);

        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3, 4]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3, 4])
    }

    #[test]
    #[cfg(feature = "e")]
    fn it_removes() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        let elem = vec.remove(1);
        dbg!(&vec);
        assert!(matches!(
            vec,
            LocalStorageVec::Stack {
                buf: [0, 2, _, _],
                len: 2
            }
        ));
        assert_eq!(elem, 1);

        let mut vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        let elem = vec.remove(1);
        assert!(matches!(vec, LocalStorageVec::Heap(..)));
        assert_eq!(vec.as_ref(), &[0, 2]);
        assert_eq!(elem, 1);
    }

    #[test]
    #[cfg(feature = "f")]
    fn it_iters() {
        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        let mut iter = vec.into_iter();
        for item in &mut iter {
            assert_eq!(item, 0);
        }
        assert_eq!(iter.next(), None);

        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 128]);
        let mut iter = vec.into_iter();
        for item in &mut iter {
            assert_eq!(item, 0);
        }
        assert_eq!(iter.next(), None);
    }
}
