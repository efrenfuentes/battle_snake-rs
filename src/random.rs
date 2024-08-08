// RandomSnake is a simple Battlesnake that moves randomly.

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};

use crate::battle_snake::BattleSnake;
use crate::model::{Board, Direction, Game, Snake};

use crate::moves::safe_moves;

pub struct RandomSnake;

impl RandomSnake {
    pub const fn new() -> Self {
        RandomSnake
    }
}

impl BattleSnake for RandomSnake {
    // info is called when you create your Battlesnake on play.battlesnake.com
    // and controls your Battlesnake's appearance
    // TIP: If you open your Battlesnake URL in a browser you should see this data
    fn info(&self) -> Value {
        info!("INFO");

        return json!({
            "apiversion": "1",
            "author": "efrenfuentes",
            "color": "#5095c7",
            "head": "do-sammy",
            "tail": "do-sammy",
        });
    }

    // start is called when your Battlesnake begins a game
    fn start(&self, _game: &Game, _turn: &i32, _board: &Board, _snake: &Snake) {
        info!("GAME START");
    }

    // end is called when your Battlesnake finishes a game
    fn end(&self, _game: &Game, _turn: &i32, _board: &Board, _snake: &Snake) {
        info!("GAME OVER");
    }

    // move is called on every turn and returns your next move
    // Valid moves are "up", "down", "left", or "right"
    // See https://docs.battlesnake.com/api/example-move for available data
    fn get_move(&self, _game: &Game, turn: &i32, board: &Board, snake: &Snake) -> Value {
        let safe_moves = safe_moves(board, snake);

        // Choose a move from the safe ones
        let chosen = self.choose_move(board, snake, safe_moves);

        info!("MOVE {}: {}", turn, chosen);
        return json!({ "move": chosen });
    }

    // Choose a random move from the safe ones
    fn choose_move<'a>(
        &self,
        _board: &Board,
        _snake: &Snake,
        safe_moves: Vec<Direction>,
    ) -> &'a str {
        safe_moves
            .choose(&mut rand::thread_rng())
            .unwrap_or(Direction::Up)
            .to_string()
            .as_str()
    }
}
