#![allow(dead_code)]

use contracts::requires;
use crate::boards::board::{Board, GameCol, GameRow};
use crate::inputs::{Observation, ObservationArray, PlayerID};
use crate::inputs::{MAX_COLS, MAX_ROWS};


pub struct BoardArray {
    board: ObservationArray,
    move_number:   u8,
    player_id:     u8,
    // observation:   Observation,
    // configuration: Configuration,
}
impl BoardArray {
    #[requires((0..MAX_COLS).contains(&col))]
    #[requires((0..MAX_ROWS).contains(&row))]
    fn get_index(col: GameCol, row: GameRow) -> usize {
        // col 0 == left | row 7-1 == right
        // row 0 == top  | row 6-1 == bottom
        (col as usize) + (row as usize) * (MAX_COLS as usize)
    }

    #[requires((0..MAX_COLS).contains(&col))]
    #[requires((0..MAX_ROWS).contains(&row))]
    #[requires([0,1,2].contains(&row))]
    fn set_index(&self, col: GameCol, row: GameRow, value: PlayerID) -> ObservationArray {
        let mut board = self.board;
        let index = Self::get_index(col, row);
        board[index] = value;
        board
    }
}

impl Board for BoardArray
{
    fn from_observation(observation: Observation) -> Self {
        let board = BoardArray {
            board:         observation.board,
            move_number:   observation.step,
            player_id:     observation.mark,
            // observation:   observation.clone(),
            // configuration
        };

        assert_eq!(board.get_move_number(), observation.step, "board.get_move_number() != observation.step");
        assert_eq!(board.get_move_player(), observation.mark, "board.get_move_player() != observation.mark");
        board
    }

    fn get_move_number(&self) -> u8 {
        self.board.iter()
            .filter(|&square| { *square != 0 as PlayerID })
            .count() as u8
    }

    fn get_square_value(&self, col: GameCol, row: GameRow) -> PlayerID {
        let index = Self::get_index(col, row);
        self.board[index]
    }

    #[requires(self.is_valid_action(action))]
    fn step(&self, action: GameCol) -> Self {
        let board = self.set_index(
            action,
            self.get_row(action).unwrap(),
            self.get_move_player()
        );
        BoardArray {
            board,
            move_number: self.get_move_number() + 1,
            player_id:   self.get_next_player(),
            // observation:   self.observation.clone(),
            // configuration: self.configuration.clone(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index() {
        let mut expected_index: usize = 0;
        for row in 0..MAX_ROWS {
            for col in 0..MAX_COLS {
                let actual_index = BoardArray::get_index(col, row);
                assert_eq!(actual_index, expected_index);
                expected_index += 1;
            }
        }
    }
}