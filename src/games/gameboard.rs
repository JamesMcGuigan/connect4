use crate::inputs::PlayerID;

pub trait GameBoard: Sized + Copy {
    type GameAction: Copy;

    // fn get_player(&self) -> PlayerID;
    // fn get_move_number(&self) -> u8;
    fn get_actions(&self, player: PlayerID) -> Vec<Self::GameAction>;
    fn step(&self, action: Self::GameAction, player: PlayerID) -> Self;
    fn terminated(&self) -> bool;
    fn winner(&self) -> Option<PlayerID>;
}