use pyo3::prelude::*;
use rand::Rng;

use crate::structs::{Configuration, Observation};

#[pyfunction]
pub fn agent_random(obs: Observation, conf: Configuration) -> u8 {
    let mut rng = rand::thread_rng();
    let mut action: u8 = rng.gen_range(0..conf.columns);
    for _attempt in 0..conf.columns {
        if obs.board[action as usize] == 0 { break; }
        action = (action + 1) % conf.columns;
    }
    action
}
