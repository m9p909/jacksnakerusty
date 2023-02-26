use battlesnake_game_types::wire_representation::Game;

use serde::{Deserialize, Serialize};

mod randoSnake;
#[derive(Serialize, Deserialize, Debug)]
pub struct AboutMe {
    apiversion: Option<String>,
    author: Option<String>,
    color: Option<String>,
    head: Option<String>,
    tail: Option<String>,
    version: Option<String>,
}

fn optstring(s: &str) -> Option<String> {
    Option::from(String::from(s))
}

fn get_default_aboutme() -> AboutMe {
    AboutMe {
        apiversion: optstring("1"),
        author: optstring("jackclarke"),
        color: optstring("#fc9300"),
        head: optstring("default"),
        tail: optstring("default"),
        version: optstring("1"),
    }
}
pub trait Snake {
    fn info(&self) -> AboutMe;
    fn start(&self, g: &Game);
    fn end(&self, g: &Game);
    fn get_move(&self, g: &Game) -> String;
}

pub fn get_random_snake() -> Box<impl Snake> {
    return Box::new(randoSnake::RandoSnake {});
}

//pub fn getMinimaxSnake() -> impl Snake {}
