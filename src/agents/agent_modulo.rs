use pyo3::prelude::*;

use crate::structs::{Configuration, Observation};

#[pyfunction]
#[allow(dead_code)]
pub fn agent_modulo(obs: Observation, conf: Configuration) -> u8 {
    let board_sum = obs.board.iter().sum::<u8>();
    let mut action = board_sum % conf.columns;
    if obs.board[action as usize] != 0 {
        for col in 0..conf.columns {
            if col != 0 { action = col }
        }
    }
    action
}
