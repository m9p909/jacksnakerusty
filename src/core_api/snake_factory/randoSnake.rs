use battlesnake_game_types::types::{Move, NeighborDeterminableGame};
use battlesnake_game_types::wire_representation::Game;

use super::{get_default_aboutme, AboutMe, Snake};
use rand::seq::SliceRandom;
pub struct RandoSnake {}

impl Snake for RandoSnake {
    fn info(&self) -> AboutMe {
        get_default_aboutme()
    }
    fn start(&self, _game: &Game) {
        println!("STARTED RANDOSNAKE");
    }
    fn get_move(&self, game: &Game) -> String {
        let pos = game.you.head;
        let res: Vec<_> = game.possible_moves(&pos).map(|m| return m.0).collect();
        println!("{:?}", res);
        let choice: Move = *res.choose(&mut rand::thread_rng()).unwrap();
        choice.to_string()
    }
    fn end(&self, _g: &Game) {
        println!("ENDED RANDOSNAKE")
    }
}
