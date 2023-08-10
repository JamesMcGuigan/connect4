use pyo3::prelude::*;

use crate::boards::Board;
use crate::inputs::{MAX_COLS, MAX_ROWS};

pub type PlayerID = u8;
pub type ObservationArray = [PlayerID; (MAX_COLS * MAX_ROWS) as usize];  // == [u8;42]

// DOCS: https://pyo3.rs/v0.13.2/class.html
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
#[allow(non_snake_case, dead_code)]
pub struct Observation {
    #[pyo3(get)] pub step: u8,
    #[pyo3(get)] pub mark: u8,
    #[pyo3(get)] pub board: ObservationArray,
    #[pyo3(get)] pub remainingOverageTime: f32,
}
#[pymethods]
#[allow(non_snake_case, dead_code)]
impl Observation {
    #[new]
    fn new(step: u8, mark: u8, board: ObservationArray, remainingOverageTime: f32) -> Self {
        Observation { step, mark, board, remainingOverageTime }
    }

    pub fn index(&self, col: u8, row: u8) -> usize { (col + (row * MAX_COLS)) as usize }

    pub fn step(&self, action: u8) -> Self {
        let col = action;
        let row= (0..MAX_ROWS)
            .rev()
            .find(|&row| self.board[self.index(col, row)] == 0 )
            .unwrap_or(0)  // BUG: will overwrite board[col][0] if col is full
        ;
        let mut board = self.board;
        board[self.index(col, row)] = self.mark;

        Observation {
            step:  self.step + 1,
            mark:  if self.mark == 1 { 2 } else { 1 },
            board,
            remainingOverageTime: self.remainingOverageTime
        }
    }
}

/// obs  = { 'remainingOverageTime': 60, 'step': 0, 'mark': 1, 'board': [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
/// conf = { 'timeout': 2, 'actTimeout': 2, 'agentTimeout': 60, 'episodeSteps': 1000, 'runTimeout': 1200, 'columns': 7, 'rows': 6, 'inarow': 4, '__raw_path__': '/kaggle_simulations/agent/main.py' }
impl Default for Observation {
    fn default() -> Self {
        Observation {
            step: 0,
            mark: 1,
            board: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            remainingOverageTime: 0.0,
        }
    }
}

impl From<ObservationArray> for Observation {
    fn from(input: ObservationArray) -> Self {
        let step = input.iter()
            .filter(|&square| { *square != 0 as PlayerID })
            .count() as u8;
        let mark = if step % 2 == 0 { 1 } else { 2 };

        Observation {
            step,
            mark,
            board: input,
            remainingOverageTime: 0.0,
        }
    }
}

impl From<Box<dyn Board>> for Observation {
    fn from(input: Box<dyn Board>) -> Observation {
        let array = input.to_array();
        Observation {
            board: array,
            step:  input.get_move_number(),
            mark:  input.get_move_player(),
            remainingOverageTime: 0.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod from_array {
        use super::*;

        #[test]
        fn test_default() {
            let input  = Observation::default();
            let output = Observation::from(input.board);
            assert_eq!(input, output);
        }

        #[test]
        fn test_from_1() {
            let input = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
            let output = Observation::from(input);
            assert_eq!(output.board, input);
            assert_eq!(output.step,  1);
            assert_eq!(output.mark,  2);
        }

        #[test]
        fn test_from_2() {
            let input = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,1];
            let output = Observation::from(input);
            assert_eq!(output.board, input);
            assert_eq!(output.step,  2);
            assert_eq!(output.mark,  1);
        }

        #[test]
        fn test_step() {
            let mut obs = Observation::default();
            assert_eq!(obs.step, 0);
            assert_eq!(obs.mark, 1);

            obs = obs.step(6);
            assert_eq!(obs.step, 1);
            assert_eq!(obs.mark, 2);
            assert_eq!(obs.board, [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]);

            obs = obs.step(5);
            assert_eq!(obs.step, 2);
            assert_eq!(obs.mark, 1);
            assert_eq!(obs.board, [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,1]);
        }
    }
}