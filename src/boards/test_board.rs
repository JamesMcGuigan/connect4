#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::boards::{Board, BoardArray};
    use crate::boards::board::GameCol;
    use crate::inputs::{MAX_COLS, MAX_ROWS, Observation, PlayerID};
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

    // fn fixture_actions_empty() -> Vec<GameCol> { vec![] }
    fn fixture_observation_empty() -> ObservationArray {[
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
    ]}
    // fn fixture_actions_col_1() -> Vec<GameCol> { vec![2,1,0,0,1,1,0,0,0,0] }
    fn fixture_observation_col_1() -> ObservationArray {[
        2,0,0,0,0,0,0,
        1,0,0,0,0,0,0,
        2,0,0,0,0,0,0,
        1,2,0,0,0,0,0,
        2,1,0,0,0,0,0,
        1,2,1,0,0,0,0,
    ]}
    fn fixture_actions_win_p1_horizontal() -> Vec<GameCol> { vec![0,6,1,6,2,6,3] }
    fn fixture_observation_win_p1_horizontal() -> ObservationArray {[
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,2,
        0,0,0,0,0,0,2,
        1,1,1,1,0,0,2,
    ]}
    fn fixture_actions_win_p2_vertical() -> Vec<GameCol> { vec![0,6,1,6,2,6,0,6] }
    fn fixture_observation_win_p2_vertical() -> ObservationArray {[
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,2,
        0,0,0,0,0,0,2,
        1,0,0,0,0,0,2,
        1,1,1,0,0,0,2,
    ]}
    fn fixture_actions_win_p1_diagonal_down() -> Vec<GameCol> { vec![3,2,2,1,0,1,0,2,1,0,0] }
    fn fixture_observation_win_p1_diagonal_down()  -> ObservationArray {[
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        1,0,0,0,0,0,0,
        2,1,2,0,0,0,0,
        1,2,1,0,0,0,0,
        1,2,2,1,0,0,0,
    ]}
    fn fixture_actions_win_p2_diagonal_up() -> Vec<GameCol> { vec![6,6,6,6,5,3,5,5,4,4] }
    fn fixture_observation_win_p2_diagonal_up()  -> ObservationArray {[
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,
        0,0,0,0,0,0,2,
        0,0,0,0,0,2,1,
        0,0,0,0,2,1,2,
        0,0,0,2,1,1,1,
    ]}
    fn fixture_observation_draw() -> ObservationArray {[
        1,2,1,2,1,2,1,
        1,2,1,2,1,2,1,
        2,1,2,1,2,1,2,
        2,1,2,1,2,1,2,
        1,2,1,2,1,2,1,
        1,2,1,2,1,2,1,
    ]}
    


    #[test]
    // All boards start at move 0 + player 1
    fn test_get_move_player_0() {
        let observation: ObservationArray = Observation::default().board;
        for board in fixture_boards(observation) {
            assert_eq!(board.get_move_number(), 0);
            assert_eq!(board.get_move_player(), 1);
        }
    }


    #[test]
    // Play a sample game along bottom row, check move_numbers and player_ids increment
    fn test_get_move_players() {
        let observation: ObservationArray = Observation::default().board;
        for mut board in fixture_boards(observation) {
            let mut expected_move = 0;
            let mut expected_player;
            for action in 0..MAX_COLS {
                expected_move  += 1;
                expected_player = board.get_next_player();
                board           = board.step(action);
                assert_eq!(board.get_move_number(), expected_move);
                assert_eq!(board.get_move_player(), expected_player);
                assert_ne!(board.get_move_player(), board.get_next_player());
            }
        }
    }


    #[test]
    // Lookup coordinates on an example board
    fn test_get_square_value() {
        let observation: ObservationArray = [
            2,0,0,0,0,0,0,
            1,0,0,0,0,0,0,
            2,0,0,0,0,0,0,
            1,2,0,0,0,0,0,
            2,1,0,0,0,0,0,
            1,2,1,0,0,0,0,
        ];
        for board in fixture_boards(observation) {
            assert_eq!(board.get_row(0), None);
            assert_eq!(board.get_square_value(0,0), 2);
            assert_eq!(board.get_square_value(0,1), 1);
            assert_eq!(board.get_square_value(0,2), 2);
            assert_eq!(board.get_square_value(0,3), 1);
            assert_eq!(board.get_square_value(0,4), 2);
            assert_eq!(board.get_square_value(0,5), 1);

            assert_eq!(board.get_row(1), Some(2));
            assert_eq!(board.get_square_value(1,2), 0);
            assert_eq!(board.get_square_value(1,3), 2);
            assert_eq!(board.get_square_value(1,4), 1);
            assert_eq!(board.get_square_value(1,5), 2);

            assert_eq!(board.get_row(2), Some(4));
            assert_eq!(board.get_square_value(2,4), 0);
            assert_eq!(board.get_square_value(2,5), 1);

            for row in 3..MAX_COLS {
                assert_eq!(board.get_row(row), Some(5));
                assert_eq!(board.get_square_value(row,5), 0);
            }
        }
    }

    #[rstest(observation, actions)]
    #[case(fixture_observation_empty(),  vec![0,1,2,3,4,5,6])]
    #[case(fixture_observation_col_1(),  vec![  1,2,3,4,5,6])]
    #[case(fixture_observation_draw(),   vec![])]
    fn test_get_valid_actions(observation: ObservationArray, actions: Vec<GameCol>) {
        for board in fixture_boards(observation) {
            assert_eq!(board.get_valid_actions(), actions);
            if actions.len() == 0 {
                assert_eq!(board.any_valid_actions(), false);
            } else {
                assert_eq!(board.any_valid_actions(), true);
            }
        }
    }

    #[rstest(observation)]
    #[case(fixture_observation_empty())]
    #[case(fixture_observation_col_1())]
    #[case(fixture_observation_win_p1_horizontal())]
    #[case(fixture_observation_win_p2_vertical())]
    #[case(fixture_observation_win_p1_diagonal_down())]
    #[case(fixture_observation_win_p2_diagonal_up())]
    #[case(fixture_observation_draw())]
    fn test_get_row(observation: ObservationArray) {
        for board in fixture_boards(observation) {
            for col in 0..MAX_COLS {
                let empty_row = board.get_row(col);
                match empty_row {
                    // Row is full, not valid action, all squares should be full
                    None => {
                        assert_eq!(board.is_valid_action(col), false);
                        for row in 0..MAX_ROWS {
                            assert_eq!(board.is_square_empty(col, row), false);
                            assert_ne!(board.get_square_value(col, row), 0);
                        }
                    },
                    // Row is partial, squares above empty_row should be empty, below are full
                    Some(empty_row) => {
                        assert_eq!(board.is_valid_action(col), true);
                        for row in 0..=empty_row {
                            assert_eq!(board.is_square_empty(col, row), true);
                            assert_eq!(board.get_square_value(col, row), 0);
                        }
                        for row in empty_row+1 .. MAX_ROWS {
                            assert_eq!(board.is_square_empty(col, row), false);
                            assert_ne!(board.get_square_value(col, row), 0);
                        }
                    }
                }
            }
        }
    }

    #[rstest(observation)]
    #[case(fixture_observation_draw())]
    fn test_board_draw(observation: ObservationArray) {
        for board in fixture_boards(observation) {
            for action in 0..MAX_COLS {
                assert_eq!(board.is_valid_action(action), false);
                assert_eq!(board.get_row(action), None);
            }
            assert_eq!(board.get_valid_actions(), vec![]);
            assert_eq!(board.any_valid_actions(), false);
            assert_eq!(board.is_draw(), true);
            assert_eq!(board.is_win(1), false);
            assert_eq!(board.is_win(2), false);
            assert_eq!(board.terminated(), true);
        }
    }

    #[rstest(name, observation, winner)]
    #[case("empty",                fixture_observation_empty(),                0)]
    #[case("col1",                 fixture_observation_col_1(),                0)]
    #[case("draw",                 fixture_observation_draw(),                 0)]    
    #[case("win_p1_horizontal",    fixture_observation_win_p1_horizontal(),    1)]
    #[case("win_p2_vertical",      fixture_observation_win_p2_vertical(),      2)]
    #[case("win_p1_diagonal_down", fixture_observation_win_p1_diagonal_down(), 1)]
    #[case("win_p2_diagonal_up",   fixture_observation_win_p2_diagonal_up(),   2)]
    fn test_board_win(name: String, observation: ObservationArray, winner: PlayerID) {
        for board in fixture_boards(observation) {
            // Debugging Output
            // cargo test -- --nocapture --test-threads=1
            const DEBUG: bool = false;
            if DEBUG {
                println!("{}", name);
                println!("is_win(p1) = {} | is_win(p2) = {} | is_draw() = {}", board.is_win(1), board.is_win(2), board.is_draw());
                println!("{}", board.to_string());
                let win_coordinates = board.winning_lines();
                win_coordinates.iter().for_each(|line| {
                    let line_values = line.iter()
                        .map(|&(col, row)| { board.get_square_value(col, row) })
                        .collect::<Vec<PlayerID>>()
                    ;
                    let line_winner = board.get_players().into_iter()
                        .find(|&player_id| line_values.iter().all(|&value| value == player_id))
                        .map(|player_id| player_id.to_string())
                        .unwrap_or("".to_string())
                    ;
                    println!("line: {:?} = {:?} = {}", line, line_values, line_winner);  // Verbose = Print every line
                    // if !line_winner.is_empty() { println!("winner: {:?} = {:?} = {}", line, line_values, line_winner); }  // Only print winner line
                });
            }

            if winner == 0 {
                assert!(!board.is_win(1), "board.is_win(p1)");
                assert!(!board.is_win(2), "board.is_win(p2)");
            } else {
                let loser: PlayerID = if winner == 1 { 2 } else { 1 };
                assert!( board.is_win(winner), "board.is_win(winner = p{})", winner);
                assert!(!board.is_win(loser),  "board.is_win(loser  = p{})", loser);
                assert!(!board.is_draw(),      "board.is_draw()");
                assert!( board.terminated(),   "board.terminated()");
            }
        }
    }

    #[rstest(name, observation, actions, winner)]
    #[case("win_p1_horizontal",    fixture_observation_win_p1_horizontal(),    fixture_actions_win_p1_horizontal(),    1)]
    #[case("win_p2_vertical",      fixture_observation_win_p2_vertical(),      fixture_actions_win_p2_vertical(),      2)]
    #[case("win_p1_diagonal_down", fixture_observation_win_p1_diagonal_down(), fixture_actions_win_p1_diagonal_down(), 1)]
    #[case("win_p2_diagonal_up",   fixture_observation_win_p2_diagonal_up(),   fixture_actions_win_p2_diagonal_up(),   2)]
    fn test_step_win(name: String, observation: ObservationArray, actions: Vec<GameCol>, winner: PlayerID) {
        let starting_boards = fixture_boards(fixture_observation_empty());
        let expected_boards = fixture_boards(observation);

        for (starting_board, expected_board) in Iterator::zip(starting_boards, expected_boards) {
            let mut board = starting_board;

            const DEBUG: bool = false;
            if DEBUG { println!("name = {}", name); }
            if DEBUG { println!("actions = {:?} | board \n{}", actions, board.to_string()); }

            for &action in &actions {
                assert!( !board.terminated(), "!board.terminated()");
                assert!(  board.is_valid_action(action), "board.is_valid_action({})", action);
                board = board.step(action);

                if DEBUG { println!("player = {} | action = {} | board \n{}", board.get_next_player(), action, board.to_string()) }
            }
            if DEBUG { println!("expected board = \n{}", expected_board.to_string()); }

            assert_eq!( board.to_string(), expected_board.to_string(), "name = {:?} | actions = {:?}", name, actions );
            assert!( board.is_win(winner), "board.is_win({})", winner);
            assert!( board.terminated(), "board.terminated()");
        }
    }

}