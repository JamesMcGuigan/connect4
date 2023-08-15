#![allow(unused_parens)]
#![allow(dead_code)]

use contracts::requires;

use crate::boards::Board;
use crate::boards::board::{GameCol, GameRow};
use crate::boards::lines::connect4_line_bitmasks;
use crate::inputs::{MAX_COLS, MAX_ROWS, Observation, PlayerID};

// Bitmask = [u42;BITS_PLAYED] + [u42;BITS_PLAYER]
pub type Bitmask = u128;  // 7*6 == 42 * 2 bits (board + player bit) == 84 bits
const BITS_PLAYED: usize = 0;
const BITS_PLAYER: usize = (MAX_COLS * MAX_ROWS) as usize;

/// Stores a Connect4 board using only u128 Bitmask
pub struct BoardBitmask {
    board: Bitmask,
    // move_number: u8,  // recompute from board
    // player_id:   u8,  // recompute from board
}

impl BoardBitmask {
    #[requires((0..MAX_COLS).contains(&col))]
    #[requires((0..MAX_ROWS).contains(&row))]
    fn get_index(col: GameCol, row: GameRow) -> usize {
        // col 0 == left | row 7-1 == right
        // row 0 == top  | row 6-1 == bottom
        ((col) + (row * MAX_COLS)) as usize
    }

    #[requires((0..MAX_COLS).contains(&col))]
    #[requires((0..MAX_ROWS).contains(&row))]
    #[requires([0,1,2].contains(&value))]
    fn set_index(&self, col: GameCol, row: GameRow, value: PlayerID) -> Bitmask {
        let mut bitboard = self.board;
        let index = Self::get_index(col, row);
        if value == 0 {
            bitboard &= !(1 << (index + BITS_PLAYED) | 1 << (index + BITS_PLAYER));  // played_bit = 0 | player_bit = 0
        } else {
            bitboard |= (1 << (index + BITS_PLAYED));  // played_bit = 1
            if value == 1 { bitboard &= !(1 << (index + BITS_PLAYER)); } // player_bit = 0
            if value == 2 { bitboard |=  (1 << (index + BITS_PLAYER)); } // player_bit = 1
        }
        bitboard
    }
}

impl From<Observation> for BoardBitmask {
    fn from(observation: Observation) -> Self {
        // u128 = u64 is_played + u64 player_id
        let mut bitboard = 0 as Bitmask;
        for col in 0..MAX_COLS as usize {
            for row in 0..MAX_ROWS as usize {
                let index = (col) + (row * MAX_COLS as usize);
                let player_id = observation.board[index];

                // Bitmask = [u42;BITS_PLAYED] + [u42;BITS_PLAYER]
                if player_id == 0 {
                    bitboard &= !(1 << (index + BITS_PLAYED));  // played_bit = 0
                    bitboard &= !(1 << (index + BITS_PLAYER));  // player_bit = 0
                } else {
                    bitboard |=  (1 << (index + BITS_PLAYED));  // played_bit = 1
                    if player_id == 1 { bitboard &= !(1 << (index + BITS_PLAYER)); } // player_bit = 0
                    if player_id == 2 { bitboard |=  (1 << (index + BITS_PLAYER)); } // player_bit = 1
                }
            }
        }

        let board = BoardBitmask {
            board:       bitboard,
            // move_number: observation.step,
            // player_id:   observation.mark,
        };

        assert_eq!(board.get_move_number(), observation.step, "board.get_move_number() != observation.step");
        assert_eq!(board.get_move_player(), observation.mark, "board.get_move_player() != observation.mark");
        board
    }
}


impl Board for BoardBitmask
{
    fn get_move_number(&self) -> u8 {
        (BITS_PLAYED..BITS_PLAYER)
            .filter(|index| (self.board & (1 << index)) != 0)
            .count() as u8
    }

    /// @Optimization
    /// Check if all played_bit's are set on top row
    /// assert all self.board[0..MAX_COLS][0] == 1
    fn any_valid_actions(&self) -> bool {
        let moves_bitmask: Bitmask = (0..MAX_COLS as usize)
            .map(|col| 1 << (col + BITS_PLAYED))
            .sum()  // 0b1111111 == 0x7F == 127
        ;
        self.board & moves_bitmask != moves_bitmask
    }

    /// @Optimization
    /// Check if the played_bit for a given index is set
    fn is_square_empty(&self, col: GameCol, row: GameRow) -> bool {
        let index = (col + row*MAX_COLS) as usize;
        let played_bit = self.board & (1 << (index + BITS_PLAYED));
        played_bit == 0
    }

    #[allow(clippy::needless_return)]
    fn get_square_value(&self, col: GameCol, row: GameRow) -> PlayerID {
        let index = (col + row*MAX_COLS) as usize;
        let played_bit = self.board & (1 << (index + BITS_PLAYED));
        let player_bit = self.board & (1 << (index + BITS_PLAYER));

        if      played_bit == 0 { return 0; }
        else if player_bit == 0 { return 1; } else { return 2; }
    }

    fn step(&self, action: GameCol) -> Option<Box<(dyn Board)>> {
        if !self.is_valid_action(action) { return None; }

        let bitboard = self.set_index(
            action,
            self.get_row(action).unwrap(),
            self.get_move_player()
        );
        Some(Box::new(Self {
            board:       bitboard,
            // move_number: self.move_number + 1,
            // player_id:   self.get_next_player(),
        }))
    }

    /// @Optimization
    /// Loop over winning_bitmasks() and check: bits_played[] == 1 & bits_player[] == player_id
    fn is_win(&self, player_id: PlayerID) -> bool {
        let lines = connect4_line_bitmasks();
        for line in lines {
            // if all bits_played are set
            if (self.board >> BITS_PLAYED) & line == line {
                // if all bits_player == player_id
                if player_id == 1 && (self.board >> BITS_PLAYER) & line == 0    { return true; }
                if player_id == 2 && (self.board >> BITS_PLAYER) & line == line { return true; }
            }
        }
        false
    }
}
