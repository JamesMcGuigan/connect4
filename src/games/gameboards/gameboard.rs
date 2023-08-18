use std::fmt;

use crate::inputs::{Configuration, Observation, PlayerID};

pub trait GameBoard: Sized + Copy + fmt::Display {
    type GameAction: Copy + fmt::Display;

    fn new(observation: Observation, configuration: Configuration) -> Self;
    fn get(&self, row: u8, col: u8) -> PlayerID;
    fn actions(&self, player: PlayerID) -> Vec<Self::GameAction>;
    fn step(&self, action: Self::GameAction, player: PlayerID) -> Option<Self>;
    fn terminated(&self) -> bool;
    fn winner(&self) -> Option<PlayerID>;

}