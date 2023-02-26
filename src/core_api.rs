use crate::core_api::snake_factory::Snake;
use battlesnake_game_types::wire_representation::Game;
use serde_json::{json, Value};
mod snake_factory;

fn get_snake() -> Box<impl Snake> {
    snake_factory::get_random_snake()
}

pub fn index() -> Value {
    let s = get_snake();
    serde_json::to_value(s.info()).unwrap()
}
pub fn start(req: &Game) {
    get_snake().start(req)
}
pub fn get_move(req: &Game) -> Value {
    let decision = get_snake().get_move(req);
    json!({ "move": decision })
}
pub fn end(req: &Game) {
    get_snake().end(req)
}
