/// This file demonstrates various binding techniques from Rust to Python
/// - random_move_column() | pass in single argument
/// - modulo_move_args()   | pass in all arguments as primitives
/// - modulo_move_struct() | map arguments to Rust structs Observation + Configuration
///
/// obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
/// conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;

// DOCS: https://pyo3.rs/v0.13.2/class.html
#[pyclass]
#[derive(Clone)]
#[allow(non_snake_case, dead_code)]
pub struct Observation {
    #[pyo3(get)] step: u8,
    #[pyo3(get)] mark: u8,
    #[pyo3(get)] board: [u8;42],
    #[pyo3(get)] remainingOverageTime: f32,
}
#[pymethods]
#[allow(non_snake_case, dead_code)]
impl Observation {
    #[new]
    fn new(step: u8, mark: u8, board: [u8;42], remainingOverageTime: f32) -> Self {
        Observation { step, mark, board, remainingOverageTime }
    }
}

#[pyclass]
#[derive(Clone)]
#[allow(non_snake_case, dead_code)]
pub struct Configuration {
    #[pyo3(get)] columns: u8,
    #[pyo3(get)] rows: u8,
    #[pyo3(get)] inarow: u8,
    #[pyo3(get)] timeout: f32,
    #[pyo3(get)] actTimeout: f32,
    #[pyo3(get)] agentTimeout: f32,
    #[pyo3(get)] episodeSteps: u32,
    #[pyo3(get)] runTimeout: f32,
    #[pyo3(get)] __raw_path__: String,
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

#[pyfunction]
pub fn random_move_column(columns: usize) -> usize {
    let mut rng = rand::thread_rng();
    let action: usize = rng.gen_range(0..columns);
    action
}


#[pyfunction]
#[allow(non_snake_case, dead_code, unused_variables, clippy::too_many_arguments)]
pub fn modulo_move_args(
    step: u8,
    mark: u8,
    board: [u8;42],
    remainingOverageTime: f32,
    columns: u8,
    rows: u8,
    inarow: u8,
    timeout: f32,
    actTimeout: f32,
    agentTimeout: f32,
    episodeSteps: u32,
    runTimeout: f32,
    __raw_path__: String,
) -> u8 {
    let obs = Observation {
        step,
        mark,
        board,
        remainingOverageTime,
    };
    let conf = Configuration {
        columns,
        rows,
        inarow,
        timeout,
        actTimeout,
        agentTimeout,
        episodeSteps,
        runTimeout,
        __raw_path__,
    };
    modulo_move_struct(obs, conf)
}


// pyfunction BUG: TypeError: argument '_obs': 'Struct' object cannot be converted to 'Observation'
#[pyfunction]
#[allow(dead_code)]
pub fn modulo_move_struct(obs: Observation, conf: Configuration) -> u8 {
    let board_sum = obs.board.iter().sum::<u8>();
    let mut action = board_sum % conf.columns;
    if obs.board[action as usize] != 0 {
        for col in 0..conf.columns {
            if col != 0 { action = col }
        }
    }
    action
}

#[pymodule]
#[pyo3(name = "maturin_kaggle")]
fn module_with_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Observation>()?;
    m.add_class::<Configuration>()?;

    m.add_wrapped(wrap_pyfunction!(random_move_column))?;
    m.add_wrapped(wrap_pyfunction!(modulo_move_args))?;
    m.add_wrapped(wrap_pyfunction!(modulo_move_struct))?;
    Ok(())
}