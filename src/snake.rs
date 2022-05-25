use std::collections::VecDeque;

#[derive(PartialEq, PartialOrd)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
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
    pub fn new(max_size_x: usize, max_size_y: usize) -> Self {
        let mut full_size = VecDeque::new();
        let pos = Position::new(max_size_x/2, max_size_y/2);
        full_size.push_front(pos.clone());
        Snake {
            pos,
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

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn move_snake(&mut self, direction: &Directions) -> Position {
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
        Snake::new(4, 4)
    }
}
