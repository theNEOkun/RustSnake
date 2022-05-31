use std::{collections::VecDeque, fmt::Display};

use crossterm::event::Event;
use tui::{
    style::{Color, Style},
    text::Span,
};

use crate::{
    board::Board,
    controller::helper_enums::{Directions, MoveOpt},
    Items,
    consts::{SNEK, EMPTY}
};

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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Happen<T> {
    Some(T),
    Break,
    None,
}

pub struct Snake<'a> {
    pos: Position,
    tail: VecDeque<Position>,
    size: usize,
    keys: fn(Event) -> MoveOpt<Directions>,
    dirr: Directions,
    snake_self: Items,
    snake: Span<'a>,
    fruit: Items,
}

impl<'a> Snake<'a> {
    /// Creates a new [`Snake`].
    pub fn new(
        start_pos: Position,
        snake_self: Items,
        fruit: Items,
        keys: fn(Event) -> MoveOpt<Directions>,
    ) -> Self {
        let mut tail = VecDeque::new();
        tail.push_front(start_pos.clone());
        let snake = match snake_self {
            Items::SNAKE => (Span::styled(SNEK, Style::default().bg(Color::Green))),
            Items::OSNAKE => (Span::styled(SNEK, Style::default().bg(Color::Yellow))),
            _ => Span::from(EMPTY),
        };

        Snake {
            pos: start_pos,
            tail,
            size: 4,
            keys,
            dirr: Directions::LEFT,
            snake_self,
            snake,
            fruit,
        }
    }

    pub fn get_back(&mut self) -> Option<Position> {
        return if self.tail.len() > self.size {
            Some(self.tail.pop_back().unwrap())
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
        self.tail.push_front(pos);
    }

    pub fn _get_size(&self) -> usize {
        self.size
    }

    pub fn get_items(&self) -> Items {
        self.snake_self.clone()
    }

    pub fn get_span(&self) -> Span {
        self.snake.clone()
    }

    fn new_pos(&self, board: &Board) -> Position {
        let pos = match self.dirr {
            Directions::UP => Position::new(self.pos.x, self.pos.y - 1),
            Directions::DOWN => Position::new(self.pos.x, self.pos.y + 1),
            Directions::LEFT => Position::new(self.pos.x - 1, self.pos.y),
            Directions::RIGHT => Position::new(self.pos.x + 1, self.pos.y),
        };
        board.get_overflow_pos(pos)
    }

    pub fn move_snake(
        &mut self,
        board: &mut Board,
        fruits: &mut Vec<(Position, Items)>,
    ) -> Happen<bool> {
        let pos = self.new_pos(board);
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
        &self.tail
    }

    pub fn get_info(&self) -> Vec<String> {
        vec![
            format!("Current size: {}", self._get_size()),
            format!("Fruits eaten: {}", self._get_size() - 4),
        ]
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

    fn make_snake<'a>() -> Snake<'a> {
        Snake::new(Position::new(4, 4), Items::SNAKE, Items::FRUIT, test_function)
    }

    fn test_function(event: Event) -> MoveOpt<Directions> {
        return MoveOpt::None
    }

    #[test]
    fn test_get_position() {
        let mut snake = make_snake();
        assert_eq!(Position::new(4, 4), snake.get_pos());
        let new_pos = snake.move_snake(&mut Board::new(4, 4, true), &mut vec![(Position::new(0, 0), Items::FRUIT)]);
        assert_eq!(Happen::None, new_pos);
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
