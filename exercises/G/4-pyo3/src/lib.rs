#![allow(unused)]

use pyo3::types::PyList;

fn main() {
    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    // TODO add a `pointwise_sum` pyfunction

    /// A Python module implemented in Rust.
    #[pymodule]
    fn pointwise_simd(py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
        // TODO re-enable when pointwise_sum is defined
        // m.add_function(wrap_pyfunction!(pointwise_sum, m)?)?;

        Ok(())
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn pointwise_sum_simd(a: &mut [f64], b: &[f64]) {
    use std::arch::x86_64::*;

    const WIDTH: usize = 2;

    let length = a.len();
    debug_assert_eq!(a.len(), b.len());

    let mut index = 0;

    while index + WIDTH <= length {
        // TODO: use simd instructions to complete the body of the loop
        //
        // useful functions
        //
        // - _mm_load_pd: load a pointer into a simd value
        // - _mm_add_pd: add two simd values
        // - _mm_storeu_pd: write a simd value to a pointer

        index += WIDTH;
    }

    while index < length {
        a[index] += b[index];

        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_pointwise_sum_simd() {
        let mut a: Vec<_> = (1..10).map(|i| i as f64).collect();
        let b: Vec<_> = (11..20).map(|i| i as f64).collect();

        unsafe { pointwise_sum_simd(&mut a, &b) };

        let expected: Vec<_> = (1..10).zip(11..20).map(|(x, y)| (x + y) as f64).collect();

        assert_eq!(a, expected);
    }
}
