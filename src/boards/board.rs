use std::ops::Range;
use crate::inputs::{MAX_COLS, MAX_ROWS, INAROW, Observation};  // Configuration
use crate::inputs::observation::PlayerID;


pub type GameRow  = u8;    // CONTRACT: 0 .. Self::get_config().rows
pub type GameCol  = u8;    // CONTRACT: 0 .. Self::get_config().columns
type GameLine = Vec<(GameCol, GameRow)>;

pub trait Board {
    // fn from_observation(observation: Observation, configuration: Configuration) -> Self;
    fn from_observation(observation: Observation) -> Self;

    // fn get_config(&self)  -> Configuration;
    fn get_players()          -> [PlayerID; 2]   { [1,2] }        // == [1,2]
    fn get_possible_actions() -> Range<GameRow>  { 0..MAX_COLS }  // == [0,1,2,3,4,5,6]

    fn get_move_number(&self) -> u8;
    fn get_move_player(&self) -> PlayerID {
        if self.get_move_number() % 2 == 0 { Self::get_players()[0] } else { Self::get_players()[1] }
    }
    fn get_next_player(&self) -> PlayerID {
        if self.get_move_player() == Self::get_players()[0] { Self::get_players()[1] } else { Self::get_players()[0] }
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

    fn is_valid_action(&self, action: GameRow) -> bool {
        if action >= MAX_COLS as GameRow { return false; }
        !self.is_square_empty(action, 0)
    }
    fn any_valid_actions(&self) -> bool {
        Self::get_possible_actions()
            .any(|action| { !self.is_valid_action(action) })
    }
    fn get_valid_actions(&self) -> Vec<GameRow> {
        Self::get_possible_actions()
            .filter(|action| { self.is_valid_action(*action) })
            .collect()
    }

    /// Play action and return copy of next board
    /// Not object-safe to define in Trait, requires Struct specific functions
    /// CONTRACT: action <= Self::get_config().columns
    /// CONTRACT: self.is_valid_action(action)
    fn step(&self, action: GameCol) -> Self;


    // Game Termination Functions

    fn winning_lines() -> Vec<GameLine> {
        let directions: Vec<(i8, i8)> = vec![
            (0, 1),  // Vertical |
            (1, 0),  // Horizontal -
            (1, 1),  // Diagonal \
            (1, -1), // Diagonal /
        ];
        let mut output: Vec<GameLine> = Vec::new();

        // Loop over each starting square on the board
        for start_col in 0..MAX_COLS as GameCol {
            for start_row in 0..MAX_ROWS as GameRow {

                // Collect Vec<GameLine> from each direction, excluding coordinate out-of-bounds
                'direction: for direction in &directions {
                    let mut line = Vec::new();
                    for offset in 0..INAROW as GameRow {
                        // i8 required for -1 negative out-of-bounds values
                        let col: i8 = start_col as i8 + offset as i8 * direction.0;
                        let row: i8 = start_row as i8 + offset as i8 * direction.1;
                        if (0..MAX_COLS as i8).contains(&col) &&
                           (0..MAX_ROWS as i8).contains(&row)
                        {
                            line.push((col as GameCol, row as GameRow));
                        } else {
                            break 'direction;  // Out-of-Bounds = Discard Line
                        }
                    }
                    assert_eq!(line.len(), INAROW as usize, "line.len() != INAROW");
                    output.push(line);  // Only add if we didn't break 'direction
                }

            }
        }
        output
    }
    fn is_win(&self, player_id: PlayerID) -> bool {
        let win_coordinates = Self::winning_lines();
        win_coordinates.iter().any(|line| {
            line.iter().all(|&(col, row)| {
                self.get_square_value(col, row) == player_id
            })
        })
    }
    fn is_draw(&self) -> bool { !Self::any_valid_actions(self) }
    fn terminated(&self) -> bool {
        self.is_draw() || Self::get_players().iter().any(|&player_id| self.is_win(player_id))
    }

}