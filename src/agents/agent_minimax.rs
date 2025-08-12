use pyo3::prelude::*;
use crate::boards::Board;
use crate::boards::BoardBitmask;
use crate::inputs::{Configuration, Observation, PlayerID};

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

    // Check for blocking opponent's winning move
    for col in board.get_valid_actions() {
        if let Some(_next_board) = board.step(col) {
            for opponent_col in board.get_valid_actions() {
                if let Some(opponent_board) = board.step(opponent_col) {
                    if col == opponent_col { continue; }  // opponent will win if we play this move
                    if opponent_board.is_win(opponent_id) {
                        return col;
                    }
                }
            }
        }
    }

    // Greedy pick huristic score for depth=1
    let mut max_score = 0;
    let mut max_col   = 3;
    for col in board.get_valid_actions() {
        if let Some(state) = board.step(col) {
            let score = minimax(&*state, depth, i32::MIN + 1, i32::MAX - 1,
                                board.get_move_player(), board.get_next_player());
            if score > max_score {
                max_score = score;
                max_col = col;
            }
        }
    }
    return max_col;
}

fn minimax(state: &dyn Board, depth: u8, mut alpha: i32, mut beta: i32,
           root_player: PlayerID, opponent_id: PlayerID) -> i32 {
    // Terminal check: depth limit, no moves, or a win on the board
    let actions = state.get_valid_actions();
    if depth == 0 || actions.is_empty() || state.terminated() {
        if state.is_win(root_player) { return i32::MAX; }
        if state.is_win(opponent_id) { return i32::MIN; }
        return state.huristic_score(root_player, opponent_id) as i32;
    }

    let is_maximizing_player = state.get_move_player() == root_player;
    if is_maximizing_player {
        let mut max_eval = i32::MIN;
        for mv in actions {
            if let Some(next) = state.step(mv) {
                let eval = minimax(&*next, depth - 1, alpha, beta, root_player, opponent_id);
                if eval > max_eval { max_eval = eval; }
                if eval > alpha    { alpha = eval; }
                if alpha >= beta   { break; } // beta cut-off
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for mv in state.get_valid_actions() {
            if let Some(next) = state.step(mv) {
                let eval = minimax(&*next, depth - 1, alpha, beta, root_player, opponent_id);
                if eval < min_eval { min_eval = eval; }
                if eval < beta     { beta = eval;     }
                if alpha >= beta   {  break;          } // alpha cut-off
            }
        }
        min_eval
    }
}