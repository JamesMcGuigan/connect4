use crate::games::gameboards::GameBoard;
use crate::games::players::PlayerID;

pub trait Agent<T: GameBoard> {
    fn act(
        &self,
        board: T,
        player: PlayerID,
        move_number: usize,
        time_remaining: f64
    ) -> T::GameAction;
}