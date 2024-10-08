// Opportunist is a simple Battlesnake that moves randomly, but gets food when it can.
use log::info;
use serde_json::{json, Value};

use crate::battle_snake::BattleSnake;
use crate::model::{Board, Direction, Game, Snake};

use crate::utils::random_move;

pub struct OpportunistSnake;

impl OpportunistSnake {
    pub const fn new() -> Self {
        OpportunistSnake
    }
}

impl BattleSnake for OpportunistSnake {
    // info is called when you create your Battlesnake on play.battlesnake.com
    // and controls your Battlesnake's appearance
    // TIP: If you open your Battlesnake URL in a browser you should see this data
    fn info(&self) -> Value {
        info!("INFO");

        json!({
            "apiversion": "1",
            "author": "efrenfuentes",
            "color": "#c47ee0",
            "head": "silly",
            "tail": "mlh-gene",
        })
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
        let head = &snake.body[0];
        let safe_moves = head.possible_directions(board, snake);

        // choose a move from the safe ones
        let chosen = self.choose_move(board, snake, safe_moves);

        info!("MOVE {}: {}", turn, chosen);
        json!({ "move": chosen })
    }

    // Get food or choose a random move from the safe ones
    fn choose_move<'a>(&self, board: &Board, snake: &Snake, safe_moves: Vec<Direction>) -> &'a str {
        let head = &snake.body[0];

        let moves_with_food = head.possible_directions_with_food(board, snake);

        if !moves_with_food.is_empty() {
            random_move(moves_with_food)
        } else {
            random_move(safe_moves)
        }
    }
}
