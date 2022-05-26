mod board;
mod snake;

use crossterm::{
    self, cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use board::Board;
use snake::{Directions, Snake};

use std::{io::stdout, thread::sleep, time::Duration};

const MAX_SIZE: usize = 16;

const WALL_STR: &str = "\x1b[0m\x1b[41m W";
const EMPTY_STR: &str = "\x1b[0m\x1b[47m  ";
const SNAKE_STR: &str = "\x1b[47m\x1b[32m S";
const FRUIT_STR: &str = "\x1b[47m\x1b[31m %";

///!Used to differentiate the different items
#[derive(PartialEq, PartialOrd, Clone)]
pub enum Items {
    WALL = 10,
    EMPTY = 0,
    SNAKE = 1,
    FRUIT = 2,
}

///used to print the board to the screen
///
///board is the board to print
///stdout is used to print
fn print_board(board: &Board, mut stdout: &std::io::Stdout) {
    execute!(stdout, Clear(ClearType::All)).unwrap();
    for (x, each) in board.get_vec().iter().enumerate() {
        let mut o_string = String::new();
        for string in each {
            o_string += match string {
                Items::WALL => WALL_STR,
                Items::FRUIT => FRUIT_STR,
                Items::SNAKE => SNAKE_STR,
                _ => EMPTY_STR,
            };
        }
        o_string += "\n\x1b[0m";
        execute!(stdout, cursor::MoveTo(0, x as u16), Print(o_string)).unwrap();
    }
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
fn gameloop(max_size: usize) {
    let stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    let mut snake = Snake::new(max_size, max_size);

    let mut board = Board::new(max_size, max_size);

    let mut dirr: Directions = Directions::LEFT;

    let mut fruit = false;

    board.fruit();

    //key detection
    loop {
        let curr_pos = snake.get_pos();
        board.change_position(&curr_pos, Items::SNAKE);

        //going to top left corner
        print_board(&board, &stdout);

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
            board.change_position(&last_pos, Items::EMPTY);
        }

        snake.set_pos(pos);

        if fruit {
            board.fruit();
            fruit = false;
        }
    }

    //disabling raw mode
    disable_raw_mode().unwrap();
}

//Main-method
//Takes arguments
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match &args[1][..] {
            "--size" => {
                let size: usize = (&args[2][..]).parse().unwrap();
                gameloop(size);
            }
            _ => gameloop(MAX_SIZE),
        }
    } else {
        gameloop(MAX_SIZE)
    }
}
