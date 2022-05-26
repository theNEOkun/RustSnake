mod board;
mod snake;
mod terminal;

use crossterm::{
    self,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
};

use board::Board;
use snake::{Directions, Snake};
use terminal::Term;

use std::{thread::sleep, time::Duration};

///!Used to differentiate the different items
#[derive(PartialEq, PartialOrd, Clone)]
pub enum Items {
    WALL = 10,
    EMPTY = 0,
    SNAKE = 1,
    FRUIT = 2,
}

fn move_snake(curr_dirr: Directions) -> Option<Directions> {
    if poll(Duration::from_millis(100)).unwrap() {
        //matching the key
        return match read().unwrap() {
            //i think this speaks for itself
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            }) => None,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                if curr_dirr != Directions::RIGHT {
                    Some(Directions::LEFT)
                } else {
                    Some(curr_dirr)
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
            }) => {
                if curr_dirr != Directions::LEFT {
                    Some(Directions::RIGHT)
                } else {
                    Some(curr_dirr)
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            }) => {
                if curr_dirr != Directions::DOWN {
                    Some(Directions::UP)
                } else {
                    Some(curr_dirr)
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
            }) => {
                if curr_dirr != Directions::UP {
                    Some(Directions::DOWN)
                } else {
                    Some(curr_dirr)
                }
            }
            _ => (
                Some(curr_dirr)
            ),
        }
    } else {
        Some(curr_dirr)
    }
}

///Main game loop
///
///param max_size is the size of max x and y
fn gameloop(mut board: Board) {

    let (max_x, max_y) = board.get_max_size();
    let mut snake = Snake::new(max_x, max_y);
    let mut term = Term::new();

    let mut dirr: Directions = Directions::LEFT;
    let mut fruit = false;

    board.fruit();

    //key detection
    loop {
        let curr_pos = snake.get_pos();
        board.change_position(&curr_pos, Items::SNAKE);

        //going to top left corner
        term.print_board(&board.get_vec());

        if let Some(new_dirr) = move_snake(dirr) {
            dirr = new_dirr;
        } else {
            break;
        }

        sleep(Duration::from_millis(50));

        let pos = snake.move_snake(&dirr);
        if !board.check_position(&pos, Items::EMPTY) {
            if board.check_position(&pos, Items::FRUIT) {
                fruit = snake.eat();
            } else {
                break;
            }
        }
        if let Some(last_pos) = snake.get_back() {
            board.remove_position(&last_pos);
        }

        snake.set_pos(pos);

        if fruit {
            board.fruit();
            fruit = false;
        }
    }

}

//Main-method
//Takes arguments
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match &args[1][..] {
            "--size" => {
                let size: usize = (&args[2][..]).parse().unwrap();
                gameloop(Board::new(size, size));
            }
            _ => gameloop(Board::default()),
        }
    } else {
        gameloop(Board::default())
    }
}
