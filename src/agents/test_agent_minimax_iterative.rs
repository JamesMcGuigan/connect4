use crate::boards::BoardBitmask;
use crate::inputs::{Observation, ObservationArray};

#[cfg(test)]
mod tests {
    use crate::agents::agent_minimax_iterative::agent_minimax_iterative;
    use crate::boards::BoardBitmask;
    use crate::inputs::{Configuration, Observation, ObservationArray};

    #[test]
    fn test_block_opponent_two_move() {
        let observation_array: ObservationArray = [
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 2, 0, 0, 0, 0,
            0, 0, 1, 0, 1, 0, 0,
            0, 1, 2, 0, 2, 2, 0,
            0, 2, 1, 1, 1, 2, 0,
        ];
        let mut obs = Observation::from(observation_array);
        obs.step = 12;
        obs.mark = 1;
        let conf = Configuration::default();

        let board = BoardBitmask::from(Observation::from(observation_array));
        let column = agent_minimax_iterative(obs, conf);

        assert_ne!(column, 3);  // column 3 is losing at it enables opponent col-3 = 4-in-a-row
    }

    #[test] // FAILS
    fn test_block_opponent_move() {
        let observation_array: ObservationArray = [
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 2, 0, 0, 0, 0,
            0, 0, 1, 0, 1, 0, 0,
            0, 1, 2, 2, 2, 2, 0,
            1, 2, 1, 1, 1, 2, 0,
        ];
        let mut obs = Observation::from(observation_array);
        obs.step = 14;
        obs.mark = 1;
        let conf = Configuration::default();

        let board = BoardBitmask::from(Observation::from(observation_array));
        let column = agent_minimax_iterative(obs, conf);

        assert_eq!(column, 3);  // now column 3 is required to block opponent move
    }
}