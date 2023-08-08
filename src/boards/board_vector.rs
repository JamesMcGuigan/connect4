#![allow(unused_imports)]
#![allow(dead_code)]

use crate::boards::Board;
use crate::boards::board::{GameCol, GameRow};
use crate::inputs::{Configuration, Observation, PlayerID};

pub struct BoardVector {
    board: Vec<Vec<PlayerID>>,
    move_number: u8,
    player_id:   u8,
}

impl From<Observation> for BoardVector {
    fn from(observation: Observation) -> Self {
        let board = BoardVector {
            board:       Vec::new(),
            move_number: observation.step,
            player_id:   observation.mark,
        };

        assert_eq!(board.get_move_number(), observation.step, "board.get_move_number() != observation.step");
        assert_eq!(board.get_move_player(), observation.mark, "board.get_move_player() != observation.mark");
        board
    }
}

impl Board for BoardVector
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
