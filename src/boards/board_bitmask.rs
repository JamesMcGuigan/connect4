#![allow(dead_code)]

use crate::boards::Board;
use crate::boards::board::{GameCol, GameRow};
use crate::inputs::{Observation, PlayerID};

type Bitmask = u128;  // 7*6 == 42 * 2 bits (board + player bit) == 84 bits

pub struct BoardBitmask {
    board: Bitmask,
    move_number: u8,
    player_id:   u8,
}

impl From<Observation> for BoardBitmask {
    fn from(observation: Observation) -> Self {
        let board = BoardBitmask {
            board:       0,
            move_number: observation.step,
            player_id:   observation.mark,
        };

        assert_eq!(board.get_move_number(), observation.step, "board.get_move_number() != observation.step");
        assert_eq!(board.get_move_player(), observation.mark, "board.get_move_player() != observation.mark");
        board
    }
}

impl Board for BoardBitmask
{
    fn get_move_number(&self) -> u8 {
        todo!()
    }

    fn get_square_value(&self, _col: GameCol, _row: GameRow) -> PlayerID {
        todo!()
    }

    fn step(&self, _action: GameCol) -> Box<(dyn Board + 'static)> {
        todo!()
    }
}
