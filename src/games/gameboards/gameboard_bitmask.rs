use std::fmt::{Display, Formatter};

use cached::proc_macro::once;
use contracts::requires;

use crate::games::gameboards::gameboard::GameBoard;
use crate::inputs::{Configuration, Observation, PlayerID};
use crate::inputs::{INAROW, MAX_COLS, MAX_ROWS};

/// This is the performance optimized version of GameBoard
/// Memory is reduced to a u128 bitmask with optimized bitshifting logic
/// Configuration is hardcoded as compile-time constants: INAROW, MAX_COLS, MAX_ROWS
/// Sized + Copy allows for stack storage
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, Debug)]
pub struct GameBoardBitmask {
    played_bits: u64,
    player_bits: u64,
}

//noinspection RsTraitImplementation
impl Display for GameBoardBitmask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = (0..MAX_ROWS).map(|row|
            (0..MAX_COLS).map(|col| self.get(col, row).to_string() )
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

impl GameBoard for GameBoardBitmask {
    type GameAction = u8;

    #[requires(configuration.columns == MAX_COLS)]
    #[requires(configuration.columns == MAX_COLS)]
    #[requires(configuration.rows    == MAX_ROWS)]
    #[requires(configuration.inarow  == INAROW)]
    #[requires(observation.board.len() == (MAX_COLS * MAX_ROWS) as usize)]
    fn new(observation: Observation, configuration: Configuration) -> Self {
        // _configuration ignored in favor of compiletime constants: MAX_COLS, MAX_ROWS, INAROW
        let mut played_bits = 0;
        let mut player_bits = 0;
        for row in 0..MAX_ROWS {
            for col in 0..MAX_COLS {
                let index = (col + row * MAX_COLS) as usize;
                let value = observation.board[index];
                if value == 0 { continue; }     // played_bit = 0 | player_bit = 0
                if value == 1 {
                    played_bits |= 1 << index;  // played_bit = 1 | player_bit = 0
                }
                if value == 2 {
                    played_bits |= 1 << index;  // played_bit = 1 | player_bit = 1
                    player_bits |= 1 << index;  // played_bit = 1 | player_bit = 1
                }
            }
        }
        Self { played_bits, player_bits }
    }

    fn get(&self, col: u8, row: u8) -> PlayerID {
        let index = col + row * MAX_COLS;
        let played_bit = (self.played_bits & (1 << index)) >> index;
        let player_bit = (self.player_bits & (1 << index)) >> index;

        let player: PlayerID = if played_bit == 0 { 0 } else if player_bit == 0 { 1 } else { 2 };
        player
    }

    /// First 7 bits represent the cols from top row of the board
    /// If the top square is empty, then this is a valid move
    fn actions(&self, _player: PlayerID) -> Vec<Self::GameAction> {
        (0..MAX_COLS)
            .filter(|col| (self.played_bits & (1 << col)) == 0 )
            .collect()
    }


    #[requires(self.actions(player).contains(&action))]
    #[requires(player == 1 || player == 2)]
    fn step(&self, action: Self::GameAction, player: PlayerID) -> Option<Self> {
        let col = action;
        let row = (0..MAX_ROWS).rev().find(|row| {
            // Search upwards from the bottom, to find the first empty played_bit
            let index = col + row * MAX_COLS;
            let played_bit = (self.played_bits & (1 << index)) >> index;
            played_bit == 0
        });
        if Option::is_none(&row) { return None; }  // assert!(self.actions(player).contains(&action));

        let index = col + (row.unwrap() * MAX_COLS);
        let played_bits = self.played_bits | (1 << index);
        let player_bits = match player {
            1 => self.player_bits & !(1 << index),  // set 0 bit
            2 => self.player_bits |  (1 << index),  // set 1 bit
            _ => unreachable!(),
        };
        Some(Self { played_bits, player_bits })
    }

    fn terminated(&self) -> bool {
        let draw_line = draw_line();
        if (self.played_bits & draw_line) == draw_line { return true; }
        if self.winner().is_some()                     { return true; }
        false
    }

    fn winner(&self) -> Option<PlayerID> {
        let win_lines = win_lines();
        for win_line in win_lines {
            if (self.played_bits & win_line) != win_line { continue; }        // not all bits have been played, skip
            if (self.player_bits & win_line) == 0        { return Some(1); }  // all bits 0 = p1 wins
            if (self.player_bits & win_line) == win_line { return Some(2); }  // all bits 1 = p2 wins
        }
        None
    }
}

/// If all (MAX_COLS*MAX_ROWS)=42 played_bits are set, we have full board and a draw
fn draw_line() -> u64 {
    2u64.pow((MAX_COLS * MAX_ROWS) as u32) - 1
}

/// Creates a list of all winning board positions, over 4 directions: horizontal, vertical and 2 diagonals
/// Python Implementation: https://github.com/JamesMcGuigan/ai-games/blob/1feee57bf932d9026f690ff4f3331d43999ede8d/games/connectx/core/ConnextXBitboard.py#L95C13-L95C13
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
#[once]
pub fn win_lines() -> Vec<u64> {
    let mut win_lines = Vec::<u64>::new();  // TODO: return [u64; 69]

    // Create a set of length INAROW=4 bitmasks for each direction starting at top left origin
    let mut mask_horizontal  = 0u64;
    let mut mask_vertical    = 0u64;
    let mut mask_diagonal_dl = 0u64;
    let mut mask_diagonal_ul = 0u64;
    for n in 0..INAROW {
        mask_horizontal  |= 1 << n;                                  // .____
        mask_vertical    |= 1 << (n * MAX_COLS);                     // `|
        mask_diagonal_dl |= 1 << (n * MAX_COLS + n);                 // `\
        mask_diagonal_ul |= 1 << (n * MAX_COLS + (INAROW - 1 - n));  // `/
    }

    // Collect line masks starting at every square on the board, excluding out-of-bounds
    let row_inner = MAX_ROWS - INAROW;
    let col_inner = MAX_COLS - INAROW;
    for row in 0..MAX_ROWS {
        for col in 0..MAX_COLS {
            let offset = col + (row * MAX_COLS);
            if col <= col_inner {
                win_lines.push(mask_horizontal << offset);
            }
            if row <= row_inner {
                win_lines.push(mask_vertical << offset);
            }
            if col <= col_inner && row <= row_inner {
                win_lines.push(mask_diagonal_dl << offset);
                win_lines.push(mask_diagonal_ul << offset);
            }
        }
    }
    win_lines
}

