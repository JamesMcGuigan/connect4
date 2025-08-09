#[cfg(test)]
mod tests {
    use crate::boards::{Board, BoardArray, BoardBitmask, BoardVector};
    use crate::inputs::{Observation};
    use crate::inputs::ObservationArray;

    // Define a fixture that provides instances of each implementation of the Board trait
    fn fixture_boards(value: ObservationArray) -> impl Iterator<Item=Box<dyn Board>> {
        let observation = Observation::from(value);
        let iter = vec![
            Box::new(BoardArray::from(observation.clone())) as Box<dyn Board>,
            Box::new(BoardVector::from(observation.clone())) as Box<dyn Board>,
            Box::new(BoardBitmask::from(observation.clone())) as Box<dyn Board>,
        ];
        iter.into_iter()
    }

    // fn fixture_actions_empty() -> Vec<GameCol> { vec![] }
    fn fixture_observation_empty() -> ObservationArray {
        [
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ]
    }


    #[test]
    // Lookup coordinates on an example board
    fn test_huristic_empty() {
        let observation_array: ObservationArray = [
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(0), 0);
        assert_eq!(board.huristic_score(1), 0);
    }

    #[test]
    fn test_huristic_corner() {
        let observation_array: ObservationArray = [
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 2,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(0), 3);
        assert_eq!(board.huristic_score(0), 3);
    }

    #[test]
    fn test_huristic_corner_wall_1() {
        let observation_array: ObservationArray = [
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(0), 1);
    }

    #[test]
    fn test_huristic_corner_wall_2() {
        let observation_array: ObservationArray = [
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(0), 11);
    }

    #[test]
    fn test_huristic_corner_wall_2b() {
        let observation_array: ObservationArray = [
            0, 1, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(1), 11);
    }

    #[test]
    fn test_huristic_corner_wall_3() {
        let observation_array: ObservationArray = [
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(0), 111);
    }

    #[test]
    fn test_huristic_corner_wall_3b() {
        let observation_array: ObservationArray = [
            0, 1, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(1), 111);
    }

    #[test]
    fn test_huristic_corner_wall_4() {
        let observation_array: ObservationArray = [
            2, 2, 0, 0, 0, 0, 0,
            2, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
            1, 2, 0, 0, 0, 0, 0,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(0), 1000);
    }

    #[test]
    fn test_huristic_corner_wall_4b() {
        let observation_array: ObservationArray = [
            1, 1, 0, 0, 0, 0, 0,
            1, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
            2, 1, 0, 0, 0, 0, 0,
        ];
        let board = BoardBitmask::from(Observation::from(observation_array));
        assert_eq!(board.huristic_score(1), 1000);
    }
}