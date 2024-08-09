mod opportunist;
mod random;

use crate::battle_snake::BattleSnake;
use opportunist::OpportunistSnake;
use random::RandomSnake;

pub fn factory(snake_type: &str) -> Box<dyn BattleSnake> {
    match snake_type {
        "opportunist" => Box::new(OpportunistSnake::new()),
        "random" => Box::new(RandomSnake::new()),
        _ => Box::new(OpportunistSnake::new()),
    }
}
