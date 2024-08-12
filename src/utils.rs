use crate::model::{Coord, Direction};
use rand::seq::SliceRandom;

#[allow(dead_code)]
pub fn distance(coord1: &Coord, coord2: &Coord) -> i32 {
    (coord1.x - coord2.x).abs() + (coord1.y - coord2.y).abs()
}

#[allow(dead_code)]
pub fn closest_point(coord: &Coord, points: &Vec<Coord>) -> Option<Coord> {
    let mut closest: Option<Coord> = None;
    let mut min_distance = i32::MAX;

    for point in points {
        let current_distance = distance(coord, point);
        if current_distance < min_distance {
            min_distance = current_distance;
            closest = Some(*point);
        }
    }

    closest
}

pub fn random_move(possible_moves: Vec<Direction>) -> &'static str {
    let direction = possible_moves
        .choose(&mut rand::thread_rng())
        .unwrap_or(&Direction::Up)
        .to_string();

    Box::leak(direction.into_boxed_str())
}
