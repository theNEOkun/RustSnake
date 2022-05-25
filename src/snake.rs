use crate::directions::Directions;

pub struct Snake {
    x_pos: u32,
    y_pos: u32,
    size: u32
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            x_pos: 0,
            y_pos: 0,
            size: 4,
        }
    }

    pub fn get_pos(&self) -> (u32, u32) {
        (self.x_pos, self.y_pos)
    }

    pub fn move_snake(&mut self, direction: Directions) {
        match direction {
            Directions::UP => {
                self.y_pos -= 1;
            },
            Directions::DOWN => {
                self.y_pos += 1;
            },
            Directions::LEFT => {
                self.x_pos -= 1;
            },
            Directions::RIGHT => {
                self.x_pos += 1;
            }
        }
    }
}

#[cfg(test)]
mod test_snake {
    use crate::snake::*;

    fn make_snake() -> Snake {
        Snake::new()
    }

    #[test]
    fn test_move_snake() {
        let mut snake = make_snake();
        assert_eq!((0, 0), snake.get_pos());
        snake.move_snake(Directions::DOWN);
        assert_eq!((0, 1), snake.get_pos());
        snake.move_snake(Directions::RIGHT);
        assert_eq!((1, 1), snake.get_pos());
        snake.move_snake(Directions::UP);
        assert_eq!((1, 0), snake.get_pos());
        snake.move_snake(Directions::LEFT);
        assert_eq!((0, 0), snake.get_pos());
    }
}
