use serde_json::Value;

use crate::model::{Board, Game, Snake};

pub trait BattleSnake {
    fn info(&self) -> Value;
    fn start(&self, game: &Game, turn: &i32, board: &Board, snake: &Snake);
    fn get_move(&self, game: &Game, turn: &i32, board: &Board, snake: &Snake) -> Value;
    fn end(&self, game: &Game, turn: &i32, board: &Board, snake: &Snake);
    fn choose_move<'a>(&self, board: &Board, snake: &Snake, safe_moves: Vec<&'a str>) -> &'a str;
}
