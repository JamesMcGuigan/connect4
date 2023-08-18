#![allow(dead_code)]

use std::sync::Arc;

use crate::games::agent::Agent;
use crate::games::gameboards::GameBoard;
use crate::games::players::{PlayerID, Players};
use crate::inputs::Configuration;



pub struct GameServer<T: GameBoard> {
    agents:  Vec<Arc<dyn Agent<T>>>,
    players: Vec<PlayerID>,
    board:   T,
    conf:    Configuration,
}
impl<T: GameBoard> GameServer<T> {
    pub fn new(agents: Vec<Arc<dyn Agent<T>>>, board: T, conf: Configuration) -> Self {
        assert_eq!(agents.len(), Players::all().len());
        Self {
            board,
            agents,
            players: Players::all(),
            conf,
        }
    }
    pub fn run(&mut self, verbose: bool) -> Option<PlayerID> {
        let time_start = std::time::Instant::now();

        let mut move_number = 0;
        while !self.board.terminated() {
            let index: usize = move_number % self.players.len();
            let player = self.players[index];
            let agent = &self.agents[index];
            let time_remaining = self.conf.timeout - time_start.elapsed().as_secs_f64();

            let action = agent.act(
                self.board,
                player,
                move_number,
                time_remaining
            );
            match self.board.step(action, player) {
                Some(board) => self.board = board,
                None => {
                    println!("illegal move: {}", action);
                    break;  // self.board.terminated() == true
                }
            }
            move_number += 1;
            if verbose {
                println!("move: {} | player: {}", move_number, player);
                println!("{}", self.board);
                println!("----------");
            }
        }
        let winner = self.board.winner();
        if verbose {
            println!("winner: {:?}", winner);
        }
        winner
    }
}