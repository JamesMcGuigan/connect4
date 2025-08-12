use pyo3::prelude::*;
use crate::boards::Board;
use crate::boards::BoardBitmask;
use crate::inputs::{Configuration, Observation};

#[pyfunction]
#[allow(dead_code)]
pub fn agent_greedy(obs: Observation, _conf: Configuration) -> u8 {
    let board: BoardBitmask = obs.into(); // Convert Observation to BoardBitmask
    let player_id = <BoardBitmask as Board>::get_move_player(&board);
    let opponent_id = <BoardBitmask as Board>::get_next_player(&board);

    // Default to column 3 (center) if first move
    if obs.step == 0 {
        return 3;
    }

    // Check for winning move
    for col in board.get_valid_actions() {
        if let Some(next_board) = board.step(col) {
            if next_board.is_win(player_id) {
                return col;
            }
        }
    }

    // Greedy pick huristic score for depth=1
    let mut max_score = 0;
    let mut max_col   = 3;
    for col in board.get_valid_actions() {
        let state = board.step(col);
        // BUGFIX: player_id for huristic_score is [0,1] not [1,2] from get_move_player()
        let score = state.unwrap().huristic_score(player_id, opponent_id);
        if score > max_score {
            max_score = score;
            max_col   = col;
        }
    }
    return max_col;
}
