#![allow(unused_imports)]
#![allow(dead_code)]
use crate::inputs::{Configuration, PlayerID};

pub struct BoardVector {
    board: Vec<Vec<PlayerID>>,
    move_number: u8,
    player_id:   u8,
}
