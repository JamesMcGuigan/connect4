#![allow(dead_code)]
type Bitmask = u128;  // 7*6 == 42 * 2 bits (board + player bit) == 84 bits

pub struct BoardBitmask {
    board: Bitmask,
    move_number: u8,
    player_id:   u8,
}
