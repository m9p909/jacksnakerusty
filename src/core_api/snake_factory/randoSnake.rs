use battlesnake_game_types::types::{Move, RandomReasonableMovesGame};
use battlesnake_game_types::wire_representation::Game;

use super::{get_default_aboutme, AboutMe, Snake};
pub struct RandoSnake {}

impl Snake for RandoSnake {
    fn info(&self) -> AboutMe {
        get_default_aboutme()
    }
    fn start(&self, _game: &Game) {
        println!("STARTED RANDOSNAKE");
    }
    fn get_move(&self, game: &Game) -> Box<String> {
        let result = &game
            .random_reasonable_move_for_each_snake(&mut rand::thread_rng())
            .filter(|data| data.0 == game.you.id)
            .map(|res| res.1.to_string())
            .collect::<Vec<String>>()[0];
        return Box::from(result.clone());
    }
    fn end(&self, _g: &Game) {
        println!("ENDED RANDOSNAKE")
    }
}
