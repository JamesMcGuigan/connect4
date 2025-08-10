use pyo3::prelude::*;
use crate::boards::Board;
use crate::boards::BoardBitmask;
use crate::inputs::{Configuration, Observation};

#[pyfunction]
#[allow(dead_code)]
pub fn agent_minimax(obs: Observation, _conf: Configuration) -> u8 {
    let depth = 5;

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
        if let Some(state) = board.step(col) {
            let score = minimax(&*state, depth, true);
            if score > max_score {
                max_score = score;
                max_col = col;
            }
        }
    }
    return max_col;
}

fn minimax(state: &dyn Board, depth: u8, is_maximizing_player: bool) -> i32 {
    // Terminal check: depth limit, no moves, or a win on the board
    let actions = state.get_valid_actions();
    if depth == 0 || actions.is_empty() || state.terminated() {
        let player = state.get_move_player();
        return state.huristic_score(player) as i32;
    }

    if is_maximizing_player {
        let mut max_eval = i32::MIN;
        for mv in actions {
            if let Some(next) = state.step(mv) {
                let eval = minimax(&*next, depth - 1, false);
                if eval > max_eval {
                    max_eval = eval;
                }
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for mv in state.get_valid_actions() {
            if let Some(next) = state.step(mv) {
                let eval = minimax(&*next, depth - 1, true);
                if eval < min_eval {
                    min_eval = eval;
                }
            }
        }
        min_eval
    }
}