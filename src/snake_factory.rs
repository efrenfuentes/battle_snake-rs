mod hungry;
mod opportunist;
mod random;

use crate::battle_snake::BattleSnake;
use hungry::HungrySnake;
use opportunist::OpportunistSnake;
use random::RandomSnake;

pub fn factory(snake_type: &str) -> Box<dyn BattleSnake> {
    match snake_type {
        "hungry" => Box::new(HungrySnake::new()),
        "opportunist" => Box::new(OpportunistSnake::new()),
        "random" => Box::new(RandomSnake::new()),
        _ => Box::new(OpportunistSnake::new()),
    }
}
