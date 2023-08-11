use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use pyo3::prelude::*;

use crate::agents::agent_random::agent_random;
use crate::boards::{Board, BoardArray};
use crate::boards::board::GameCol;
use crate::inputs::{Configuration, Observation, MAX_COLS};

lazy_static! {
    static ref HISTORY: Mutex<HashMap<u8, Vec<Observation>>> = Mutex::new(HashMap::new());
}

/// This is an inefficient brute-force 2-deep search
/// Find our + opponent's action to step() into the current board
fn get_opponent_action(obs: Observation, last_obs: Option<Observation>) -> Option<GameCol> {
    let start_board = match last_obs {
        Some(last_obs) => { BoardArray::from(last_obs) }
        None => { BoardArray::from(Observation::default()) }
    };

    for action_p1 in 0..MAX_COLS {
        if let Some(board_p1) = start_board.step(action_p1) {
            if board_p1.to_array() == obs.board {
                return Some(action_p1);
            }
            for action_p2 in 0..MAX_COLS {
                if let Some(board_p2) = board_p1.step(action_p2) {
                    if board_p2.to_array() == obs.board {
                        return Some(action_p2);
                    }
                }
            }
        }
    }
    None
}

/// agent_mirror()
/// Attempt to mirror the opponent's last move, else play 3=middle or agent_random()
#[pyfunction]
pub fn agent_mirror(obs: Observation, conf: Configuration) -> u8 {
    // hold history_lock until end-of-scope GC
    let mut history_lock = HISTORY.lock().unwrap();

    // Extract .last() Observation from HISTORY
    let last_obs = history_lock
        .entry(obs.mark)
        .or_insert_with(Vec::new)
        .last()
        .cloned()  // .map(|obs| obs.clone()) == cast Option<&Observation> -> Option<Observation>
    ;
    // Add new Observation to HISTORY
    history_lock.entry(obs.mark)
        .or_insert_with(Vec::new)
        .push(obs.clone())
    ;

    // Actual agent logic
    agent_mirror_action(obs, conf, last_obs)
}

pub fn agent_mirror_action(obs: Observation, conf: Configuration, last_obs: Option<Observation>) -> u8 {
    // Find opponent action and counter with mirror move
    let opponent_action = get_opponent_action(obs.clone(), last_obs);
    let mut action = match opponent_action {
        Some(opponent_action) => { (MAX_COLS-1) - opponent_action },  // mirror column
        None => { (MAX_COLS-1) / 2 }                                      // middle column == 3
    };

    // Validate this is a legal move, else play random
    let board = BoardArray::from(obs.board);
    while !board.is_valid_action(action) {
        action = agent_random(obs.clone(), conf.clone());
    };
    action
}


#[cfg(test)]
mod test {
    use crate::inputs::MAX_ROWS;
    use super::*;

    fn clear_history() {
        HISTORY.lock().unwrap().clear();
    }

    /// assert get_opponent_action(*, None) == Some(*) not None for all 0..MAX_COLS
    #[test]
    fn test_get_opponent_action_none() {
        for action in 0..MAX_COLS {
            let board = BoardArray::from(Observation::default()).step(action).unwrap();
            let obs = Observation::from(board);
            let opponent_action = get_opponent_action(obs, None);
            assert_eq!(opponent_action, Some(action));
        }
    }

    // prove get_opponent_action() can reverse-engineer action_p2 given next_board+last_board
    #[test]
    fn test_get_opponent_action_empty() {
        let start_board = BoardArray::from(Observation::default());
        for action_p1 in 0..MAX_COLS {
            for action_p2 in 0..MAX_COLS {
                let last_board = start_board.step(action_p1).unwrap();
                let next_board = last_board.step(action_p2).unwrap();
                let opponent_action = get_opponent_action(
                    Observation::from(next_board),
                    Some(Observation::from(last_board))
                );
                assert_eq!(opponent_action, Some(action_p2));
            }
        }
    }


    /// assert agent_mirror() == 3 for first move
    #[test]
    fn test_move_0() {
        clear_history();
        let action = agent_mirror(Observation::default(), Configuration::default());
        assert_eq!(action, 3);  // default action is 3
    }

    /// Play 2-agent game until filled board
    /// test agent_mirror() starts middle=3, counters middle=3, then detects when column in full
    /// DEBUG shows agent_mirror() plays 3, 3, 3, 3, 3, 3:full, 6:rand, 0:mirror, 6, 0...
    #[test]
    fn test_move_col3() {
        clear_history();
        let mut obs = Observation::default();

        // start center 3 + then counter 3 until col is full
        for row in 0..MAX_ROWS*MAX_COLS {
            let action = agent_mirror(obs.clone(), Configuration::default());
            obs = obs.step(action);

            const DEBUG: bool = true;
            if DEBUG { print!("{},  ", action); }  // 3, 3, 3, 3, 3, 3:full, 6:rand, 0:mirror, 6, 0, 6, 0, 6, 0, 6, 0, 6, 0:full, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 5:full, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1,

            if row < MAX_ROWS {
                assert_eq!(action, 3);  // start center 3 + then counter 3 until col is full
            } else {
                assert_ne!(action, 3);  // col 3 is full now, mirror play elsewhere
            }
        }
    }

    #[test]
    fn test_move_mirror() {
        const MIRROR_MOVES: [(GameCol, GameCol); 7] = [ (0,6), (1,5), (2,4), (3,3), (4,2), (5,1), (6,0) ];
        for (action_p1, action_p2) in  MIRROR_MOVES {
            clear_history();
            assert_eq!(HISTORY.lock().unwrap().len(), 0, "HISTORY.len() != 0");

            // step action_p1 (without HISTORY); assert agent_mirror() == action_p2 mirror move
            let mut obs= Observation::default().step(action_p1);
            let mut action = agent_mirror(obs.clone(), Configuration::default());
            assert_eq!(HISTORY.lock().unwrap().len(), 1, "HISTORY.len() != 1");
            assert_eq!(action, action_p2, "{} mirrors {}", action_p1, action_p2);
    
            // step action_p2 (with HISTORY); assert agent_mirror() == action_p1 mirror move
            obs    = obs.step(action_p2);
            action = agent_mirror(obs.clone(), Configuration::default());
            assert_eq!(HISTORY.lock().unwrap().len(), 2, "HISTORY.len() != 2");
            assert_eq!(action, action_p1, "{} mirrors {}", action_p1, action_p2);
        }
    }
}