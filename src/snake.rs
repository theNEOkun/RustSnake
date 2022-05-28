use std::{collections::VecDeque, fmt::Display};

use crossterm::event::Event;

use crate::{board::Board, terminal::MoveOpt, Items};

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }
}

pub enum Happen<T> {
    Some(T),
    Break,
    None,
}

pub struct Snake {
    pos: Position,
    full_size: VecDeque<Position>,
    size: usize,
    keys: fn(Event) -> MoveOpt<Directions>,
    dirr: Directions,
    snake_self: Items,
    fruit: Items,
}

impl Snake {
    /// Creates a new [`Snake`].
    pub fn new(
        start_pos: Position,
        snake_self: Items,
        fruit: Items,
        keys: fn(Event) -> MoveOpt<Directions>,
    ) -> Self {
        let mut full_size = VecDeque::new();
        full_size.push_front(start_pos.clone());
        Snake {
            pos: start_pos,
            full_size,
            size: 4,
            keys,
            dirr: Directions::LEFT,
            snake_self,
            fruit,
        }
    }

    pub fn get_back(&mut self) -> Option<Position> {
        return if self.full_size.len() >= self.size {
            Some(self.full_size.pop_back().unwrap())
        } else {
            None
        };
    }

    pub fn mover(&mut self, read: Event) {
        let opt = self.keys;
        let dirr = match read {
            event => opt(event),
        };
        if let MoveOpt::Some(new_dirr) = dirr {
            if self.dirr != opposite(&new_dirr) {
                self.dirr = new_dirr
            }
        }
    }

    pub fn eat(&mut self) -> bool {
        self.size += 1;
        true
    }

    pub fn fruit(&self) -> &Items {
        &self.fruit
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

    pub fn get_items(&self) -> Items {
        self.snake_self.clone()
    }

    pub fn move_snake(
        &mut self,
        board: &mut Board,
        fruits: &mut Vec<(Position, Items)>,
    ) -> Happen<bool> {
        let pos = match self.dirr {
            Directions::UP => Position::new(self.pos.x, self.pos.y - 1),
            Directions::DOWN => Position::new(self.pos.x, self.pos.y + 1),
            Directions::LEFT => Position::new(self.pos.x - 1, self.pos.y),
            Directions::RIGHT => Position::new(self.pos.x + 1, self.pos.y),
        };
        let pos = board.get_overflow_pos(pos);
        //return if !(board.check_position(&pos, &Items::WALL) || board.check_position(&pos, &self.snake_self)) {
        return if !board.check_position(&pos, &Items::EMPTY) {
            for fruit_pos in 0..fruits.len() {
                let each = &fruits[fruit_pos];
                if &pos == &each.0 && &self.fruit == &each.1 {
                    fruits.remove(fruit_pos);
                    self.set_pos(pos);
                    return Happen::Some(self.eat());
                }
            }
            Happen::Break
        } else {
            self.set_pos(pos);
            Happen::None
        };
    }

    pub fn get_tail(&self) -> &VecDeque<Position> {
        &self.full_size
    }
}

/// Method used to get the opposite direction of a given direction
fn opposite(dirr: &Directions) -> Directions {
    return match dirr {
        Directions::LEFT => Directions::RIGHT,
        Directions::RIGHT => Directions::LEFT,
        Directions::UP => Directions::DOWN,
        Directions::DOWN => Directions::UP,
    };
}

#[cfg(test)]
mod test_snake {
    use crate::snake::*;

    fn make_snake() -> Snake {
        Snake::new(Position::new(4, 4), vec![])
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
