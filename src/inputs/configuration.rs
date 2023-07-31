use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
#[allow(non_snake_case, dead_code)]
pub struct Configuration {
    #[pyo3(get)] pub columns: u8,
    #[pyo3(get)] pub rows: u8,
    #[pyo3(get)] pub inarow: u8,
    #[pyo3(get)] pub episodeSteps: u32,
    #[pyo3(get)] pub timeout: f32,
    #[pyo3(get)] pub actTimeout: f32,
    #[pyo3(get)] pub agentTimeout: f32,
    #[pyo3(get)] pub runTimeout: f32,
    #[pyo3(get)] pub __raw_path__: String,
}
#[pymethods]
#[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
impl Configuration {
    #[new]
    fn new(
        columns: u8, rows: u8, inarow: u8, episodeSteps: u32,
        timeout: f32, actTimeout: f32, agentTimeout: f32,  runTimeout: f32,
        __raw_path__: String
    ) -> Self {
        Configuration {
            columns, rows, inarow, episodeSteps,
            timeout, actTimeout, agentTimeout,  runTimeout,
            __raw_path__
        }
    }
}

/// Kaggle ConnectX Competition Configuration is in practice a constant
/// obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
/// conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }
impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            columns: 7,
            rows: 6,
            inarow: 4,
            episodeSteps: 1000,
            timeout: 2.0,
            actTimeout: 2.0,
            agentTimeout: 60.0,
            runTimeout: 1200.0,
            __raw_path__: "/kaggle_simulations/agent/main.py".to_string(),
        }
    }
}