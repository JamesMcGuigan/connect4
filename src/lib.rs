use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pyfunction]
fn double_f64(x: f64) -> f64 {
    x * 2.0
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
#[pyo3(name = "maturin_kaggle")]
fn module_with_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(double))?;
    m.add_wrapped(wrap_pyfunction!(double_f64))?;
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    Ok(())
}