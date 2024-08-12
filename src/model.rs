use rocket::serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

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

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    pub fn distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn is_adjacent(&self, other: &Coord) -> bool {
        self.distance(other) == 1
    }

    pub fn is_valid(&self, board: &Board) -> bool {
        self.x >= 0 && self.x < board.width && self.y >= 0 && self.y < board.height
    }

    pub fn is_safe(&self, board: &Board, snake: &Snake) -> bool {
        !board.hazards.contains(self)
            && !board.snakes.iter().any(|s| s.body.contains(self))
            && !snake.body.contains(self)
    }

    pub fn neighbors(&self) -> Vec<Coord> {
        vec![
            Coord {
                x: self.x + 1,
                y: self.y,
            },
            Coord {
                x: self.x - 1,
                y: self.y,
            },
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }

    pub fn possible_moves(&self, board: &Board, snake: &Snake) -> Vec<Coord> {
        self.neighbors()
            .into_iter()
            .filter(|c| c.is_valid(board) && c.is_safe(board, snake))
            .collect()
    }

    pub fn possible_directions(&self, board: &Board, snake: &Snake) -> Vec<Direction> {
        self.possible_moves(board, snake)
            .into_iter()
            .map(|c| self.direction_to(&c))
            .collect()
    }

    pub fn possible_moves_with_food(&self, board: &Board, snake: &Snake) -> Vec<Coord> {
        self.possible_moves(board, snake)
            .into_iter()
            .filter(|c| board.food.contains(c))
            .collect()
    }

    pub fn possible_directions_with_food(&self, board: &Board, snake: &Snake) -> Vec<Direction> {
        self.possible_moves_with_food(board, snake)
            .into_iter()
            .map(|c| self.direction_to(&c))
            .collect()
    }

    pub fn closest_food(&self, board: &Board) -> Option<Coord> {
        board.food.iter().min_by_key(|f| self.distance(f)).cloned()
    }

    pub fn closest_safe(&self, board: &Board, snake: &Snake) -> Option<Coord> {
        self.neighbors()
            .iter()
            .filter(|c| c.is_valid(board) && c.is_safe(board, snake))
            .min_by_key(|c| self.distance(c))
            .cloned()
    }

    pub fn direction_to(&self, other: &Coord) -> Direction {
        if self.x == other.x {
            if self.y < other.y {
                Direction::Up
            } else {
                Direction::Down
            }
        } else if self.y == other.y {
            if self.x < other.x {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            Direction::Up
        }
    }

    pub fn move_to(&self, direction: &Direction) -> Coord {
        match direction {
            Direction::Up => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Snake,
}

// The direction the snake is moving
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Convert a string to a direction
    ///
    /// # Arguments
    ///
    /// * `s` - A string to convert to a direction
    ///
    /// # Returns
    ///
    /// A direction if the string is a valid direction, otherwise None
    ///
    /// # Example
    ///
    /// ```
    /// use crate::model::Direction;
    ///
    /// let up = Direction::from_str("up");
    ///
    /// assert_eq!(up, Some(Direction::Up));
    /// ```
    pub fn from_str(s: &str) -> Option<Direction> {
        match s {
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            "left" => Some(Direction::Left),
            "right" => Some(Direction::Right),
            _ => None,
        }
    }

    /// Get the opposite direction
    ///
    /// # Returns
    ///
    /// The opposite direction
    ///
    /// # Example
    ///
    /// ```
    /// use crate::model::Direction;
    ///
    /// let up = Direction::Up;
    /// let down = up.opposite();
    ///
    /// assert_eq!(down, Direction::Down);
    /// ```
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// Convert a direction to a coordinate
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin coordinate
    ///
    /// # Returns
    ///
    /// A new coordinate based on the direction
    ///
    /// # Example
    ///
    /// ```
    /// use crate::model::{Coord, Direction};
    ///
    /// let origin = Coord { x: 0, y: 0 };
    /// let up = Direction::Up;
    /// let new_coord = up.to_coord(&origin);
    ///
    /// assert_eq!(new_coord, Coord { x: 0, y: 1 });
    /// ```
    pub fn to_coord(&self, origin: &Coord) -> Coord {
        match self {
            Direction::Up => Coord {
                x: origin.x,
                y: origin.y + 1,
            },
            Direction::Down => Coord {
                x: origin.x,
                y: origin.y - 1,
            },
            Direction::Left => Coord {
                x: origin.x - 1,
                y: origin.y,
            },
            Direction::Right => Coord {
                x: origin.x + 1,
                y: origin.y,
            },
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
        }
    }
}

// Test Direction to String
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_to_string() {
        assert_eq!(Direction::Up.to_string(), "up");
        assert_eq!(Direction::Down.to_string(), "down");
        assert_eq!(Direction::Left.to_string(), "left");
        assert_eq!(Direction::Right.to_string(), "right");
    }

    #[test]
    fn test_direction_from_string() {
        assert_eq!(Direction::from_str("up"), Some(Direction::Up));
        assert_eq!(Direction::from_str("down"), Some(Direction::Down));
        assert_eq!(Direction::from_str("left"), Some(Direction::Left));
        assert_eq!(Direction::from_str("right"), Some(Direction::Right));
        assert_eq!(Direction::from_str("invalid"), None);
    }

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
    }

    #[test]
    fn test_direction_to_coord() {
        let origin = Coord { x: 0, y: 0 };

        assert_eq!(Direction::Up.to_coord(&origin), Coord { x: 0, y: 1 });
        assert_eq!(Direction::Down.to_coord(&origin), Coord { x: 0, y: -1 });
        assert_eq!(Direction::Left.to_coord(&origin), Coord { x: -1, y: 0 });
        assert_eq!(Direction::Right.to_coord(&origin), Coord { x: 1, y: 0 });
    }
}
