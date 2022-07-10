use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;


#[pyfunction]
fn random_move(columns: usize) -> usize {
    let mut rng = rand::thread_rng();
    let action: usize = rng.gen_range(0..columns);
    action
}

#[pymodule]
#[pyo3(name = "maturin_kaggle")]
fn module_with_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(random_move))?;
    Ok(())
}