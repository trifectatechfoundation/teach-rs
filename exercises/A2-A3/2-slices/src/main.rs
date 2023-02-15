// This a unfinished implementation of the well-known merge sort algorithm
//
// 1. Fix the language problems in the function merge
//
// 2. Finish the implementation of the function merge_sort
//
// 3. try changing the type from i32 into String everywhere; does your program still compile? What changes are necessary?

/// Merge two array slices (that have to be sorted) into a vector
fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut dest = Vec::new();

    let a_idx = 0;
    let b_idx = 0;

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] <= b[b_idx] {
            dest.push(a[a_idx]);
            a_idx += 1
        } else {
            dest.push(b[b_idx]);
            b_idx += 1
        }
    }

    for elem in a[a_idx..] {
        dest.push(elem)
    }
    for elem in b[b_idx..] {
        dest.push(elem)
    }

    dest
}

/// Take an array slice, and sort into a freshly constructor vector using the above function
fn merge_sort(data: &[i32]) -> Vec<i32> {
    if data.len() > 1 {
	// implement this
	todo!()
    } else {
        data.to_vec()
    }
}

/// Read a bunch of numbers from standard input into a Vec<i32>.
fn read_numbers() -> Vec<i32> {
    use std::io;
    let mut result = Vec::new();
    for line in io::stdin().lines().flatten() {
        for word in line.split_whitespace() {
            result.push(word.parse().unwrap())
        }
    }

    result
}

fn main() {
    let input = read_numbers();
    println!("Data to be sorted:");
    println!("{input:?}");

    let sorted_input = merge_sort(&input);
    println!("Sorted data:");
    println!("{sorted_input:?}");
}
