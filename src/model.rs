use rocket::serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Snake>,
    pub hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Snake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: i32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Snake,
}
