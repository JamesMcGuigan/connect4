use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;

// obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
// conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }

#[pyclass]
#[derive(Clone)]
#[allow(non_snake_case, dead_code)]
pub struct Observation {
    step: u32,
    mark: u8,
    board: [u8;42],
    remainingOverageTime: f32,
}

#[pyclass]
#[derive(Clone)]
#[allow(non_snake_case, dead_code)]
pub struct Configuration {
    columns: u8,
    rows: u8,
    inarow: u8,
    timeout: f32,
    actTimeout: f32,
    agentTimeout: f32,
    episodeSteps: u32,
    runTimeout: f32,
    // __raw_path__: String,
}

#[pyfunction]
pub fn random_move(columns: usize) -> usize {
    let mut rng = rand::thread_rng();
    let action: usize = rng.gen_range(0..columns);
    action
}

#[pyfunction]
#[allow(non_snake_case, dead_code, unused_variables, clippy::too_many_arguments)]
pub fn random_move_args(
    step: u32,
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
) -> usize {
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
    };
    random_move(conf.columns as usize)
}

// pyfunction BUG: TypeError: argument '_obs': 'Struct' object cannot be converted to 'Observation'
#[pyfunction]
#[allow(dead_code)]
pub fn random_move_struct(obs: Observation, conf: Configuration) -> u8 {
    let board_sum = obs.board.iter().sum::<u8>();
    let action = board_sum % conf.columns;
    action
}

#[pymodule]
#[pyo3(name = "maturin_kaggle")]
fn module_with_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(random_move))?;
    m.add_wrapped(wrap_pyfunction!(random_move_args))?;
    Ok(())
}