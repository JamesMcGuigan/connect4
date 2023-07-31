use std::ops::Range;
use crate::inputs::Configuration;


type PlayerID = u8;    // CONTRACT: [0,1]
type GameRow  = u8;    // CONTRACT: 0 .. Self::get_config().rows
type GameCol  = u8;    // CONTRACT: 0 .. Self::get_config().columns


pub trait Board {
    fn get_config(&self)  -> Configuration;
    fn get_players(&self) -> Range<PlayerID> { 0..2 }                               // == [0,1]
    fn get_actions(&self) -> Range<GameRow>  { 0..Self::get_config(self).columns }  // == [0,1,2,3,4,5,6]

    fn get_move_number(&self) -> u8;
    fn get_move_player(&self) -> PlayerID {
        // even move 0 % 2 == player 0
        // odd  move 1 % 2 == player 1
        self.get_move_number() % self.get_players().len() as u8
    }

    fn get_col_height(&self, col: GameCol) -> Option<GameRow>;
    fn get_square_value(&self, col: GameCol, row: GameRow) -> Option<PlayerID>;
    fn is_square_empty(&self,  col: GameCol, row: GameRow) -> bool {
        self.get_square_value(col, row).is_none()
    }
    fn set_square_value(&self, col: GameCol, row: GameRow, value: Option<PlayerID>) -> Self;

    fn is_valid_action(&self, action: GameRow) -> bool;
    fn any_valid_actions(&self) -> bool {
        Self::get_actions(self)
            .any(|action| { !self.is_valid_action(action) })
    }
    fn get_valid_actions(&self) -> Vec<GameRow> {
        Self::get_actions(self)
            .filter(|action| { self.is_valid_action(*action) })
            .collect()
    }

    fn is_win(&self, player_id: PlayerID) -> bool;
    fn is_draw(&self) -> bool { !Self::any_valid_actions(self) }
    fn terminated(&self) -> bool {
        self.is_draw() || self.get_players().any(|player_id| { self.is_win(player_id) })
    }

    /// Play action and return copy of next board
    /// Not object-safe to define in Trait, requires Struct specific functions
    /// CONTRACT: action <= Self::get_config().columns
    /// CONTRACT: self.is_valid_action(action)
    fn step(&self, action: GameCol) -> Self;
}