#[cfg(test)]
mod tests {
    // use rstest::rstest;

    use crate::boards::{Board, BoardArray};
    // use crate::boards::{BoardBitmask, BoardVector};
    use crate::inputs::{MAX_COLS, Observation};
    use crate::inputs::ObservationArray;

    // Define a fixture that provides instances of each implementation of the Board trait
    fn fixture_boards(value: ObservationArray) -> impl Iterator<Item = Box<dyn Board>> {
        let observation = Observation::from(value);
        let iter = vec![
            Box::new(BoardArray::from(  observation.clone())) as Box<dyn Board>,
            // Box::new(BoardVector::from( observation.clone())) as Box<dyn Board>,
            // Box::new(BoardBitmask::from(observation.clone())) as Box<dyn Board>,
        ];
        iter.into_iter()
    }


    #[test]
    fn test_get_move_number_0() {
        let observation: ObservationArray = Observation::default().board;
        for board in fixture_boards(observation) {
            assert_eq!(board.get_move_number(), 0);
        }
    }

    #[test]
    fn test_get_move_number() {
        let observation: ObservationArray = Observation::default().board;
        for mut board in fixture_boards(observation) {
            let mut step = 0;
            for action in 0..MAX_COLS {
                println!("action {} ({})", action, board.is_valid_action(action));
                board = board.step(action);
                step += 1;
                assert_eq!(board.get_move_number(), step);
            }
        }
    }
}