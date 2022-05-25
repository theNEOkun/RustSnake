mod snake;
mod board;

use crossterm::{
    self,
    execute,
    cursor,
    event::{read, poll, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use snake::{Snake, Directions};
use board::Board;

use std::{
    io::{
        stdout, Write
    },
    time::Duration,
    thread::sleep,
};

const MAX_SIZE: usize = 16;

const WALL_Str: &str = " #";
const EMPTY_Str: &str = "  ";
const SNAKE_Str: &str = " 0";
const FRUIT_Str: &str = " %";

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Items {
    WALL = 10,
    EMPTY = 0,
    SNAKE = 1,
    FRUIT = 2,
}

fn print(o_str: &str, mut stdout: &std::io::Stdout) {

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(o_str))
        .unwrap();
}

fn print_board(board: &Board, mut stdout: &std::io::Stdout) {
    execute!(stdout, Clear(ClearType::All)).unwrap();
    for (x, each) in board.get_vec().iter().enumerate() {
        let mut o_string = String::new();
        for string in each {
            o_string += match string {
                Items::WALL => WALL_Str,
                Items::FRUIT => FRUIT_Str,
                Items::SNAKE => SNAKE_Str,
                _ => EMPTY_Str
            };
        }
        o_string += "\n";
        execute!(stdout, cursor::MoveTo(0, x as u16), Print(o_string))
            .unwrap();
        }
}

fn main() {
    let stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    print(r#"q to exit"#, &stdout);

    let mut snake = Snake::new(MAX_SIZE, MAX_SIZE);

    let mut board = Board::new(MAX_SIZE, MAX_SIZE);

    let mut dirr: Directions = Directions::LEFT;

    let mut fruit = false;

    board.fruit();

    //key detection
    loop {
        let curr_pos = snake.get_pos();
        board.change_position(&curr_pos, Items::SNAKE);

        if fruit {
            board.fruit();
            fruit = false;
        }

        //going to top left corner
        print_board(&board, &stdout);

        if poll(Duration::from_millis(100)).unwrap() {
            //matching the key
            match read().unwrap() {
                //i think this speaks for itself
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    //clearing the screen and printing our message
                }) => if dirr != Directions::RIGHT {
                    dirr = Directions::LEFT
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                }) =>
                if dirr != Directions::LEFT {
                    dirr = Directions::RIGHT
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                }) =>
                if dirr != Directions::DOWN {
                    dirr = Directions::UP
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                }) => if dirr != Directions::UP {
                    dirr = Directions::DOWN
                },
                _ => (
                ),
            }
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
    }

    //disabling raw mode
    disable_raw_mode().unwrap();
}
