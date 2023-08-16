#![allow(dead_code)]

use std::sync::Arc;
use crate::games::agent::Agent;
use crate::games::players::{PlayerID, Players};
use crate::games::gameboard::GameBoard;

struct GameServer<T: GameBoard> {
    board:   T,
    agents:  Vec<Arc<dyn Agent<T>>>,
    players: Vec<PlayerID>,
}
impl<T: GameBoard> GameServer<T> {
    pub fn new(board: T, agents: Vec<Arc<dyn Agent<T>>>) -> Self {
        assert_eq!(agents.len(), Players::all().len());
        Self {
            board,
            agents,
            players: Players::all()
        }
    }
    pub fn run(&mut self) -> Option<PlayerID> {
        let time_remaining = 0.0;
        let mut move_number = 0;
        while !self.board.terminated() {
            let index: usize = move_number % self.players.len();
            let player = self.players[index];
            let agent = &self.agents[index];

            let action = agent.get_action(
                self.board,
                player,
                move_number,
                time_remaining
            );
            self.board = self.board.step(action, player);
            move_number += 1;
        }
        self.board.winner()
    }
}