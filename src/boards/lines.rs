#![allow(unused_parens)]

use crate::boards::board::{GameCol, GameRow};
use crate::inputs::{INAROW, MAX_COLS, MAX_ROWS};
use cached::proc_macro::once;
use crate::boards::Bitmask;

pub type GameLine = Vec<(GameCol, GameRow)>;

const DIRECTIONS: [(i8, i8); 4] = [
    (0, 1),  // Vertical |
    (1, 0),  // Horizontal -
    (1, 1),  // Diagonal \
    (1, -1), // Diagonal /
];

/// Returns a list of all possible lines of length INAROW on a connect4 board
/// [ [(0,0),(0,1),(0,2),(0,3)], ... ]
/// ```
/// use std::collections::HashSet;
/// use connectx::boards::lines::connect4_lines;
/// use connectx::inputs::INAROW;
///
/// let lines = connect4_lines();
/// assert_eq!( lines.len(), 69, "4×6=24 horizontal + 3×7=21 vertical + (3×4)=12*2=24 diagonal = 69 lines" );
/// assert_eq!( lines.iter().collect::<HashSet<_>>().len(), 69, "win_lines: no duplicates" );
/// assert!(    lines.iter().all(|line| line.len() == INAROW as usize), "win_lines: 4 bits in every row" );
/// ```
#[once]
pub fn connect4_lines() -> Vec<GameLine> {
    let mut output: Vec<GameLine> = Vec::new();
    for direction in &DIRECTIONS {
        // Loop over each starting square on the board
        for start_row in 0..MAX_ROWS as GameRow {
            for start_col in 0..MAX_COLS as GameCol {

                let mut line = Vec::new();
                for offset in 0..INAROW as GameRow {
                    // i8 required for -1 negative out-of-bounds values
                    let col: i8 = start_col as i8 + (direction.0 * offset as i8);
                    let row: i8 = start_row as i8 + (direction.1 * offset as i8);
                    if (0..MAX_COLS as i8).contains(&col) &&
                       (0..MAX_ROWS as i8).contains(&row)
                    {
                        line.push((col as GameCol, row as GameRow));
                    } else {
                        break;  // then: line.len() != INAROW
                    }
                }
                if line.len() == INAROW as usize {
                    output.push(line);  // Only add if we have a full line
                }
            }
        }
    }
    output
}

/// Convert connect4_lines() into u42 bitmasks
#[once]
pub fn connect4_line_bitmasks() -> Vec<Bitmask> {
    let mut output: Vec<Bitmask> = Vec::new();
    let lines = connect4_lines();
    for line in lines {
        let mut bitmask = 0 as Bitmask;
        for (col, row) in line {
            let index = (col + (row * MAX_COLS)) as usize;
            bitmask |= (1 << index);
        }
        output.push(bitmask);
    }
    output
}
