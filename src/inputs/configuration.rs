use std::hash::{Hash, Hasher};

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_snake_case, dead_code)]
pub struct Configuration {
    #[pyo3(get)] pub columns: u8,
    #[pyo3(get)] pub rows: u8,
    #[pyo3(get)] pub inarow: u8,
    #[pyo3(get)] pub episodeSteps: u32,
    #[pyo3(get)] pub timeout: f64,
    #[pyo3(get)] pub actTimeout: f64,
    #[pyo3(get)] pub agentTimeout: f64,
    #[pyo3(get)] pub runTimeout: f64,
    // #[pyo3(get)] pub __raw_path__: &'static str, // &'static str required for const Configuration::default()
}
#[pymethods]
#[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
impl Configuration {
    #[new]
    fn new(
        columns: u8, rows: u8, inarow: u8, episodeSteps: u32,
        timeout: f64, actTimeout: f64, agentTimeout: f64,  runTimeout: f64,
        // __raw_path__: String  // #[pymethods] requires String (not str)
    ) -> Self {
        Configuration {
            columns, rows, inarow, episodeSteps,
            timeout, actTimeout, agentTimeout,  runTimeout,
            // __raw_path__,  // const requires &static str (not String.to_str())
        }
    }

    /// Kaggle ConnectX Competition Configuration is in practice a constant
    /// obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
    /// conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }
    #[staticmethod]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        <Self as Default>::default()
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
            // __raw_path__: "/kaggle_simulations/agent/main.py",
        }
    }
}


impl Eq for Configuration {}

impl Hash for Configuration {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.columns.hash(state);
        self.rows.hash(state);
        self.inarow.hash(state);
        self.episodeSteps.hash(state);
        self.timeout.to_bits().hash(state);      // hashing for f64
        self.actTimeout.to_bits().hash(state);   // hashing for f64
        self.agentTimeout.to_bits().hash(state); // hashing for f64
        self.runTimeout.to_bits().hash(state);   // hashing for f64
        // self.__raw_path__.hash(state);
    }
}


// Compile time constants are required for static array sizing
pub const MAX_COLS: u8 = 7;  // Configuration::default().columns as usize;  // MAX_COLS = 7
pub const MAX_ROWS: u8 = 6;  // Configuration::default().rows    as usize;  // MAX_ROWS = 6
pub const INAROW:   u8 = 4;  // Configuration::default().inarow  as usize;  // INAROW   = 4
pub const TIMEOUT:  f64 = 2.0;  // Configuration::default().timeout         // TIMEOUT  = 4