use crate::model::{Board, Coord, Direction, Snake};
use std::collections::HashMap;

pub fn safe_moves(board: &Board, snake: &Snake) -> Vec<Direction> {
    let mut is_move_safe = init_safe_moves();

    // Prevent your Battlesnake from moving backwards
    let head = &snake.body[0];
    let neck = &snake.body[1];

    check_neck_and_head(neck, head, &mut is_move_safe);

    // Prevent your Battlesnake from moving out of bounds
    let board_width = &board.width;
    let board_height = &board.height;

    check_board_bounds(board_width, board_height, head, &mut is_move_safe);

    // Prevent your Battlesnake from colliding with itself
    let body = &snake.body;

    check_snake_body(body, head, &mut is_move_safe);

    // Prevent your Battlesnake from colliding with other Battlesnakes
    let opponents = &board.snakes;

    check_snakes_bodies(opponents, head, &mut is_move_safe);

    // Are there any safe moves left?
    is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>()
}

fn init_safe_moves() -> HashMap<Direction, bool> {
    vec![
        (Direction::Up, true),
        (Direction::Down, true),
        (Direction::Left, true),
        (Direction::Right, true),
    ]
    .into_iter()
    .collect()
}

fn check_neck_and_head(neck: &Coord, head: &Coord, is_move_safe: &mut HashMap<Direction, bool>) {
    if neck.x < head.x {
        // Neck is left of head, don't move left
        is_move_safe.insert(Direction::Left, false);
    } else if neck.x > head.x {
        // Neck is right of head, don't move right
        is_move_safe.insert(Direction::Right, false);
    } else if neck.y < head.y {
        // Neck is below head, don't move down
        is_move_safe.insert(Direction::Down, false);
    } else if neck.y > head.y {
        // Neck is above head, don't move up
        is_move_safe.insert(Direction::Up, false);
    }
}

fn check_board_bounds(
    width: &i32,
    height: &i32,
    head: &Coord,
    is_move_safe: &mut HashMap<Direction, bool>,
) {
    if head.x == 0 {
        // Head is on the left edge of the board, don't move left
        is_move_safe.insert(Direction::Left, false);
    } else if head.x == width - 1 {
        // Head is on the right edge of the board, don't move right
        is_move_safe.insert(Direction::Right, false);
    }

    if head.y == 0 {
        // Head is on the bottom edge of the board, don't move down
        is_move_safe.insert(Direction::Down, false);
    } else if head.y == height - 1 {
        // Head is on the top edge of the board, don't move up
        is_move_safe.insert(Direction::Up, false);
    }
}

fn check_snake_body(snake: &[Coord], head: &Coord, is_move_safe: &mut HashMap<Direction, bool>) {
    for body_part in snake {
        if head.x == body_part.x {
            if head.y == body_part.y - 1 {
                is_move_safe.insert(Direction::Up, false);
            } else if head.y == body_part.y + 1 {
                is_move_safe.insert(Direction::Down, false);
            }
        } else if head.y == body_part.y {
            if head.x == body_part.x - 1 {
                is_move_safe.insert(Direction::Right, false);
            } else if head.x == body_part.x + 1 {
                is_move_safe.insert(Direction::Left, false);
            }
        }
    }
}

fn check_snakes_bodies(
    snakes: &[Snake],
    head: &Coord,
    is_move_safe: &mut HashMap<Direction, bool>,
) {
    for snake in snakes {
        let snake_body = &snake.body;

        check_snake_body(snake_body, head, is_move_safe);
    }
}
