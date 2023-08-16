use crate::games::gameboard::GameBoard;
use crate::games::players::PlayerID;

pub trait Agent<T: GameBoard> {
    fn get_action(
        &self,
        board: T,
        player: PlayerID,
        move_number: usize,
        time_remaining: f64
    ) -> T::GameAction;
}