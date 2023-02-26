#[macro_use]
extern crate rocket;

use battlesnake_game_types::wire_representation::Game;
use log::info;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

mod core_api;

#[get("/")]
fn handle_index() -> Json<Value> {
    return Json(core_api::index());
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<Game>) -> Status {
    core_api::start(&start_req.into_inner());
    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<Game>) -> Json<Value> {
    Json(core_api::get_move(&move_req.into_inner()))
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<Game>) -> Status {
    core_api::end(&end_req.into_inner());
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    // Lots of web hosting services expect you to bind to the port specified by the `PORT`
    // environment variable. However, Rocket looks at the `ROCKET_PORT` environment variable.
    // If we find a value for `PORT`, we set `ROCKET_PORT` to that value.
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", &port);
    }

    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!("Starting Battlesnake Server...");

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "battlesnake/github/starter-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}
