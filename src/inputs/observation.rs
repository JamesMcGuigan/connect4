use pyo3::prelude::*;
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
}

#[allow(dead_code)]
impl Observation {
    fn from_array(board: ObservationArray) -> Self {
        let step = board.iter()
            .filter(|&square| { *square != 0 as PlayerID })
            .count() as u8;
        let mark = if step % 2 == 0 { 1 } else { 2 };

        Observation {
            step,
            mark,
            board,
            remainingOverageTime: 0.0,
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


#[cfg(test)]
mod tests {
    use super::*;

    mod from_array {
        use super::*;

        #[test]
        fn test_default() {
            let input  = Observation::default();
            let output = Observation::from_array(input.board);
            assert_eq!(input, output);
        }

        #[test]
        fn test_first_move() {
            let input = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
            let output = Observation::from_array(input);
            assert_eq!(output.board, input);
            assert_eq!(output.step,  1);
            assert_eq!(output.mark,  2);
        }

        #[test]
        fn test_second_move() {
            let input = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,1];
            let output = Observation::from_array(input);
            assert_eq!(output.board, input);
            assert_eq!(output.step,  2);
            assert_eq!(output.mark,  1);
        }
    }
}