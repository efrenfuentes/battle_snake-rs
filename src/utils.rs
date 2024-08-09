use crate::model::{Coord, Direction};

pub fn move_to_coord(moves: &Direction, head: &Coord) -> Coord {
    match moves {
        Direction::Up => Coord {
            x: head.x,
            y: head.y + 1,
        },
        Direction::Down => Coord {
            x: head.x,
            y: head.y - 1,
        },
        Direction::Left => Coord {
            x: head.x - 1,
            y: head.y,
        },
        Direction::Right => Coord {
            x: head.x + 1,
            y: head.y,
        },
    }
}

#[allow(dead_code)]
pub fn coordinate_to_move(coord: &Coord, head: &Coord) -> Direction {
    if coord.x == head.x && coord.y == head.y + 1 {
        Direction::Up
    } else if coord.x == head.x && coord.y == head.y - 1 {
        Direction::Down
    } else if coord.x == head.x - 1 && coord.y == head.y {
        Direction::Left
    } else if coord.x == head.x + 1 && coord.y == head.y {
        Direction::Right
    } else {
        Direction::Up
    }
}
