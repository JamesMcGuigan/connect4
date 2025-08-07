use pyo3::prelude::*;
use crate::boards::Board;
use crate::boards::BoardBitmask;
use crate::inputs::{Configuration, Observation};

#[pyfunction]
#[allow(dead_code)]
pub fn agent_greedy(obs: Observation, _conf: Configuration) -> u8 {
    let board: BoardBitmask = obs.into(); // Convert Observation to BoardBitmask
    let player_id = <BoardBitmask as Board>::get_move_player(&board);

    // // Check for winning move
    // for col in board.get_valid_actions() {
    //     if let Some(next_board) = board.step(col) {
    //         if next_board.is_win(player_id) {
    //             return col;
    //         }
    //     }
    // }

    // Greedy pick huristic score for depth=1
    let chosen_col = board
        .get_valid_actions()
        .into_iter()
        .filter_map(|col|
            board.step(col)
                .map(|b| (col, b.huristic_score(player_id)))
        )
        .max_by_key(|&(_, score)| score)
        .map(|(col, _)| col)
        .unwrap_or(3);  // Default to column 3 (center) if no valid actions

    return chosen_col;
}
