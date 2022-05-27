use std::{
collections::VecDeque,
fmt::Display,
};

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Position {
    pub x: isize,
    pub y: isize
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position {
            x, y
        }
    }
}

pub struct Snake {
    pos: Position,
    full_size: VecDeque<Position>,
    size: usize
}

impl Snake {
    /// Creates a new [`Snake`].
    pub fn new(start_pos: Position) -> Self {
        let mut full_size = VecDeque::new();
        full_size.push_front(start_pos.clone());
        Snake {
            pos: start_pos,
            full_size,
            size: 4,
        }
    }

    pub fn get_back(&mut self) -> Option<Position> {
        return if self.full_size.len() >= self.size {
            Some(self.full_size.pop_back().unwrap())
        } else {
            None
        }
    }

    pub fn eat(&mut self) -> bool {
        self.size += 1;
        true
    }

    pub fn get_pos(&self) -> Position {
        self.pos.clone()
    }

    pub fn set_pos(&mut self, pos: Position) {
        self.pos = pos.clone();
        self.full_size.push_front(pos);
    }

    pub fn _get_size(&self) -> usize {
        self.size
    }

    pub fn move_snake(&self, direction: &Directions) -> Position {
        return match direction {
            Directions::UP => {
                Position::new(self.pos.x, self.pos.y - 1)
            },
            Directions::DOWN => {
                Position::new(self.pos.x, self.pos.y + 1)
            },
            Directions::LEFT => {
                Position::new(self.pos.x - 1, self.pos.y)
            },
            Directions::RIGHT => {
                Position::new(self.pos.x + 1, self.pos.y)
            }
        }
    }
}

#[cfg(test)]
mod test_snake {
    use crate::snake::*;

    fn make_snake() -> Snake {
        Snake::new(Position::new(4, 4))
    }

    #[test]
    fn test_get_position() {
        let snake = make_snake();
        assert_eq!(Position::new(2, 2), snake.get_pos());
        let new_pos = snake.move_snake(&Directions::DOWN);
        assert_eq!(Position::new(2, 3), new_pos);
    }

    #[test]
    fn test_eat() {
        let mut snake = make_snake();
        assert_eq!(4, snake.size);
        assert!(snake.eat());
        assert_eq!(5, snake.size);
        assert!(snake.eat());
        assert_eq!(6, snake.size);
        assert!(snake.eat());
        assert_eq!(7, snake.size);
    }
}
