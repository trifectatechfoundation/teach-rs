#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
// Points to ./some_mod.rs
mod some_mod;
// Points to ./another_mod/mod.rs
mod another_mod;
// Imports an item defined in ./another_mod/mod.rs
use another_mod::Item;

pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

pub fn add_generic<O, T: std::ops::Add<Output = O>>(x: T, y: T) -> O {
    x + y
}

/// Some very large struct
pub struct LargeStruct {
    data: [u8; 4096],
}

/// Takes owned [LargeStruct] and returns it when done
pub fn manipulate_large_struct(mut large: LargeStruct) -> LargeStruct {
    todo!()
}

/// Just borrows [LargeStruct]
pub fn manipulate_large_struct_borrowed(large: &mut LargeStruct) {
    todo!()
}

fn enable_led(enabled: bool) {
    todo!()
}

enum LedState {
    Enabled,
    Disabled,
}

fn set_led_state(state: LedState) {
    todo!()
}

fn do_stuff_with_led() {
    enable_led(true);
    set_led_state(LedState::Enabled)
}

/// A well-documented struct.
/// ```rust
/// # // lines starting with a `#` are hidden
/// # use ex_b::MyDocumentedStruct;
/// let my_struct = MyDocumentedStruct {
///     field: 1,
/// };
/// println!("{:?}", my_struct.field);
/// ```
pub struct MyDocumentedStruct {
    /// A field with data
    pub field: u32,
}

/// Public module
/// Accessible from outside
pub mod my_pub_mod {
    /// Private module
    /// Only accessible from parent module
    mod private_mod {

        /// Public struct
        /// Accessible wherever `private_mod` is
        pub struct PubStruct {
            field: u32,
        }
    }

    /// Private struct
    /// Only accessible from current and child modules
    struct PrivStruct {
        field: private_mod::PubStruct,
    }
}

/// Swaps two values at the `first` and `second` indices of the slice
fn slice_swap_items(slice: &mut [u32], first: usize, second: usize) {
    let tmp = slice[second];
    slice[second] = slice[first];
    slice[first] = tmp;
}

/// This module is only compiled in `test` configuration
#[cfg(test)]
mod tests {
    use crate::slice_swap_items;

    // Mark function as test
    #[test]
    fn test_swap_items() {
        let mut array = [0, 1, 2, 3, 4, 5];
        slice_swap_items(&mut array, 1, 4);
        assert_eq!(array, [0, 4, 2, 3, 1, 5]);
    }

    #[test]
    // This should panic due to out-of-bounds access
    #[should_panic]
    fn test_swap_oob() {
        let mut array = [0, 1, 2, 3, 4, 5];
        slice_swap_items(&mut array, 1, 6);
    }
}
