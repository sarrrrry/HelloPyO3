use pyo3::prelude::*;

#[pyfunction]
fn sum_and_times_100(a: i32, b: i32) -> PyResult<i32> {
    Ok(100 * a + 100 * b)
}

#[pymodule]
fn rust_backend(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_and_times_100, m)?)?;
    Ok(())
}

