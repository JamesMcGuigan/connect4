use std::fmt::{Display, Formatter};

use cached::proc_macro::cached;
use contracts::requires;

use crate::games::gameboards::gameboard::GameBoard;
use crate::inputs::{Configuration, Observation, PlayerID};



/// Simplest implementation of GameBoard using a 1D Array + move_number + player_id
/// Observation + Configuration are copied in full for every instance
/// .step() is implemented in Observation
/// Configuration is copied as part of the data structure and accessed at Runtime
/// A reference implementation and unoptimised performance comparison against GameBoardBitmask
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, Debug)]
pub struct GameBoardArray {
    obs:  Observation,
    conf: Configuration,
}

//noinspection RsTraitImplementation
impl Display for GameBoardArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = (0..self.conf.rows).map(|row|
            (0..self.conf.columns).map(|col| self.get(col, row).to_string() )
                .collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", string)
    }

    //// BUG: https://youtrack.jetbrains.com/issue/CPP-34783/Rust-impl-Display-missing-tostring
    //// CLion: Not all trait items implemented, missing: `to_string` [E0046]
    //// Rustc: error[E0407]: method `to_string` is not a member of trait `Display`
    // fn to_string(&self) -> String {
    //     todo!()
    // }
}

impl GameBoard for GameBoardArray {
    type GameAction = u8;

    #[requires(obs.board.len() == (conf.columns * conf.rows) as usize)]
    fn new(obs: Observation, conf: Configuration) -> Self {
        Self { obs, conf }
    }

    #[requires(col < self.conf.columns)]
    #[requires(row < self.conf.rows)]
    fn get(&self, col: u8, row: u8) -> PlayerID {
        let index = (col + row * self.conf.columns) as usize;
        self.obs.board[index]
    }

    /// First 7 bits represent the cols from top row of the board
    /// If the top square is empty, then this is a valid move
    fn actions(&self, _player: PlayerID) -> Vec<Self::GameAction> {
        (0..self.conf.columns)
            .filter(|&col| self.obs.board[col as usize] == 0 )
            .collect()
    }


    #[requires(self.actions(player).contains(&action))]
    #[requires(player == 1 || player == 2)]
    fn step(&self, action: Self::GameAction, player: PlayerID) -> Option<Self> {
        if !self.actions(player).contains(&action) { return None; }

        let col = action;
        let row = (0..self.conf.rows)
            .rev()
            .find(|&row| self.obs.board[self.index(col, row)] == 0 )
            .unwrap_or(0)  // BUG: will overwrite board[col][0] if col is full
        ;
        let mut board = self.obs.board;
        board[self.index(col, row)] = self.obs.mark;

        Some(Self {
            obs: Observation {
                step:  self.obs.step + 1,
                mark:  if self.obs.mark == 1 { 2 } else { 1 },
                board,
                remainingOverageTime: self.obs.remainingOverageTime
            },
            conf: self.conf,
        })
    }

    fn terminated(&self) -> bool {
        // Draw if all squares are filled
        let max_steps = self.conf.columns * self.conf.rows;
        if self.obs.step >= max_steps { return true; }
        if self.winner().is_some()    { return true; }
        false
    }


    fn winner(&self) -> Option<PlayerID> {
        let win_lines:  Vec<Vec<(u8, u8)>> = win_lines(self.conf);
        let win_values: Vec<Vec<u8>> = win_lines.iter().map(|line| {
            line.iter().map(|(col, row)| self.get(*col, *row)).collect()
        }).collect::<Vec<_>>();

        for win_value in win_values {
            if win_value.iter().any(|&value| value == 0) { continue; }
            if win_value.iter().all(|&value| value == 1) { return Some(1); }
            if win_value.iter().all(|&value| value == 2) { return Some(2); }
        }
        None
    }
}

impl GameBoardArray {
    pub fn index(&self, col: u8, row: u8) -> usize { (col + (row * self.conf.columns)) as usize }
}

/// Creates a list of all winning board positions, over 4 directions: horizontal, vertical and 2 diagonals
/// This is a slightly inefficient algorithm but it get cached
/// ```
/// use std::collections::HashSet;
/// use connectx::games::gameboards::gameboard_bitmask::win_lines;
/// use connectx::inputs::INAROW;
///
/// let win_lines = win_lines();
/// assert_eq!( win_lines.len(), 69, "4×6=24 horizontal + 3×7=21 vertical + (3×4)=12*2=24 diagonal = 69 lines" );
/// assert_eq!( win_lines.iter().collect::<HashSet<_>>().len(), 69, "win_lines: no duplicates" );
/// assert!(    win_lines.iter().all(|&line| line.count_ones() == INAROW as u32), "win_lines: 4 bits in every row" );
/// ```
#[cached]
pub fn win_lines(conf: Configuration) -> Vec<Vec<(u8, u8)>> {
    const DIRECTIONS: [(i8, i8); 4] = [
        (0, 1),  // Vertical |
        (1, 0),  // Horizontal -
        (1, 1),  // Diagonal \
        (1, -1), // Diagonal /
    ];
    let inarow  = conf.inarow  as i8;
    let rows    = conf.rows    as i8;
    let columns = conf.columns as i8;

    // Loop spans +-inarow to catch (1, -1) diagonal that starts bottom-left rather than top-left
    let mut win_lines = Vec::<Vec<(u8, u8)>>::new();
    for row in -inarow .. rows + inarow {
        for col in -inarow .. columns + inarow {
            for direction in DIRECTIONS {
                // Create lines in all directions from all squares
                // i8 is required to handle negative directions
                let line: Vec<(i8, i8)> = (0..inarow).map(|n| (
                    col + direction.0 * n,
                    row + direction.1 * n,
                )).collect();

                // Filter any that are out-of-bounds
                if line.iter().all(|(col, row)| {
                    (0..columns).contains(col) &&
                    (0..rows).contains(row)
                }) {
                    // Cast back to u8 for export
                    let line: Vec<(u8, u8)> = line.iter()
                        .map(|(col, row)| (*col as u8, *row as u8))
                        .collect();
                    win_lines.push(line);
                }
            }
        }
    }
    win_lines
}
