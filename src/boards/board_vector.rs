#![allow(unused_imports)]
#![allow(dead_code)]

use contracts::requires;

use crate::boards::Board;
use crate::boards::board::{GameCol, GameRow};
use crate::inputs::{Configuration, MAX_COLS, MAX_ROWS, Observation, PlayerID};

/// Similar to BoardArray but uses a 2D Vec rather than a 1D Array
/// move_number + player_id are recomputed from board without struct storage
pub struct BoardVector {
    board: Vec<Vec<PlayerID>>,
    // move_number: u8,
    // player_id:   u8,
}

impl From<Observation> for BoardVector {
    #[allow(clippy::needless_range_loop)]
    fn from(observation: Observation) -> Self {
        // cast: [u8;42] -> board[col][row]
        let mut board: Vec<Vec<PlayerID>> = vec![vec![0; MAX_ROWS as usize]; MAX_COLS as usize];
        for col in 0..MAX_COLS as usize {
            for row in 0..MAX_ROWS as usize {
                let index = col + (row * MAX_COLS as usize);
                board[col][row] = observation.board[index];
            }
        }
        BoardVector {
            board,
            // move_number: observation.step,
            // player_id:   observation.mark,
        }
    }
}

impl Board for BoardVector
{
    fn get_move_number(&self) -> u8 {
        // self.board.iter()
        //     .flatten()
        //     .filter(|&&x| x != 0)
        //     .count() as u8

        self.board.iter()
            .map(|row|
                row.iter()
                    .filter(|&&x| x != 0)
                    .count() as u8
            )
            .sum()
    }


    #[requires((0..MAX_COLS).contains(&col))]
    #[requires((0..MAX_ROWS).contains(&row))]
    fn get_square_value(&self, col: GameCol, row: GameRow) -> PlayerID {
        self.board[col as usize][row as usize]
    }

    // #[requires(self.is_valid_action(action))]
    fn step(&self, action: GameCol) -> Option<Box<(dyn Board)>> {
        if !self.is_valid_action(action) { return None; }

        let col = action as usize;
        let row = self.get_row(action).unwrap() as usize;
        let player_id = self.get_move_player();
        let mut board = self.board.clone();
        board[col][row] = player_id;

        Some(Box::new(Self {
            board,
            // move_number: self.get_move_number() + 1,
            // player_id:   self.get_next_player(),
            // observation:   self.observation.clone(),
            // configuration: self.configuration.clone(),
        }))
    }
}
