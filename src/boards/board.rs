use std::ops::Range;
use crate::boards::lines::connect4_lines;

use crate::inputs::{MAX_COLS, MAX_ROWS};
use crate::inputs::observation::PlayerID;

pub type GameRow  = u8;    // CONTRACT: 0 .. Self::get_config().rows
pub type GameCol  = u8;    // CONTRACT: 0 .. Self::get_config().columns


pub trait Board {
    // fn from_observation(observation: Observation, configuration: Configuration) -> Self;
    // fn from_observation(observation: Observation) -> Self;

    // fn get_config(&self)  -> Configuration;
    fn get_players(&self)          -> [PlayerID; 2]   { [1,2] }        // == [1,2]
    fn get_possible_actions(&self) -> Range<GameRow>  { 0..MAX_COLS }  // == [0,1,2,3,4,5,6]

    fn get_move_number(&self) -> u8;
    fn get_move_player(&self) -> PlayerID {
        if self.get_move_number() % 2 == 0 { self.get_players()[0] } else { self.get_players()[1] }
    }
    fn get_next_player(&self) -> PlayerID {
        if self.get_move_player() == self.get_players()[0] { self.get_players()[1] } else { self.get_players()[0] }
    }


    // Getters / Setters

    fn get_square_value(&self, col: GameCol, row: GameRow) -> PlayerID;
    fn is_square_empty(&self,  col: GameCol, row: GameRow) -> bool {
        self.get_square_value(col, row) == 0
    }

    /// Returns the row of the first empty square in the column | 0 == top
    /// Searches from the bottom up and returns index of first empty row
    fn get_row(&self, col: GameCol) -> Option<GameRow> {
        if !self.is_valid_action(col) { return None; }
        (0..MAX_ROWS as GameRow).rev()
            .find(|&row| self.is_square_empty(col, row))
    }


    // Validation Functions

    fn is_valid_action(&self, action: GameCol) -> bool {
        (0..MAX_COLS as GameCol).contains(&action) && self.is_square_empty(action, 0)
    }
    fn any_valid_actions(&self) -> bool {
        self.get_possible_actions()
            .any(|action| { self.is_valid_action(action) })
    }
    fn get_valid_actions(&self) -> Vec<GameCol> {
        self.get_possible_actions()
            .filter(|action| { self.is_valid_action(*action) })
            .collect()
    }

    /// Play action and return copy of next board
    /// Not object-safe to define in Trait, requires Struct specific functions
    /// CONTRACT: action <= Self::get_config().columns
    /// CONTRACT: self.is_valid_action(action)
    fn step(&self, action: GameCol) -> Box<dyn Board>;


    // Game Termination Functions

    fn is_win(&self, player_id: PlayerID) -> bool {
        let lines = connect4_lines();
        lines.iter().any(|line| {
            line.iter().all(|&(col, row)| {
                self.get_square_value(col, row) == player_id
            })
        })
    }
    fn is_draw(&self) -> bool { !Self::any_valid_actions(self) }
    fn terminated(&self) -> bool {
        self.is_draw() || self.get_players().iter().any(|&player_id| self.is_win(player_id))
    }

    fn to_string(&self) -> String {
        (0..MAX_ROWS).map(|row|
            (0..MAX_COLS)
                .map(|col| self.get_square_value(col, row).to_string() )
                .collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}