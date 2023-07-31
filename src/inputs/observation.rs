
use pyo3::prelude::*;

// DOCS: https://pyo3.rs/v0.13.2/class.html
#[pyclass]
#[derive(Clone)]
#[allow(non_snake_case, dead_code)]
pub struct Observation {
    #[pyo3(get)] pub step: u8,
    #[pyo3(get)] pub mark: u8,
    #[pyo3(get)] pub board: [u8;42],
    #[pyo3(get)] pub remainingOverageTime: f32,
}
#[pymethods]
#[allow(non_snake_case, dead_code)]
impl Observation {
    #[new]
    fn new(step: u8, mark: u8, board: [u8;42], remainingOverageTime: f32) -> Self {
        Observation { step, mark, board, remainingOverageTime }
    }
}
