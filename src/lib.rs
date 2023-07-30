use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::agents::agent_modulo::agent_modulo;
use crate::agents::agent_random::agent_random;
use crate::inputs::{Configuration, Observation};

pub mod agents;
pub mod inputs;

#[pymodule]
#[pyo3(name = "connectx")]
fn module_with_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    // obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
    // conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }
    m.add_class::<Observation>()?;
    m.add_class::<Configuration>()?;

    m.add_wrapped(wrap_pyfunction!(agent_random))?;
    m.add_wrapped(wrap_pyfunction!(agent_modulo))?;
    Ok(())
}