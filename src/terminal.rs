use crossterm::{
    self, cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crate::Directions;
use crate::Items;

pub struct Term {
    stdout: Stdout,
}

const WALL: &str = " W";
const FRUIT: &str = " %";
const SNEK: &str = " S";
const EMPTY: &str = "  ";

pub enum MoveOpt<T> {
    Some(T),
    Same,
    None
}

impl Term {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        Term { stdout: stdout() }
    }

    ///used to print the board to the screen
    ///
    ///board is the board to print
    ///stdout is used to print
    pub fn print_board(&mut self, matrix: &Vec<Vec<Items>>) {
        execute!(self.stdout, Clear(ClearType::All)).unwrap();
        for (x, each) in matrix.iter().enumerate() {
            let mut o_string = String::new();
            for string in each {
                o_string += &match string {
                    Items::WALL => WALL.white().on_red().to_string(),
                    Items::FRUIT => FRUIT.red().on_white().to_string(),
                    Items::SNAKE => SNEK.black().on_green().to_string(),
                    _ => EMPTY.white().on_white().to_string(),
                };
            }
            o_string += "\n\x1b[0m";
            execute!(self.stdout, cursor::MoveTo(0, x as u16), Print(o_string)).unwrap();
        }
    }

    /// Method used to move the snake
    pub fn move_snake(&self) -> MoveOpt<Directions> {
        if poll(Duration::from_millis(100)).unwrap() {
            //matching the key
            return match read().unwrap() {
                //i think this speaks for itself
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                }) => return MoveOpt::None,
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    //clearing the screen and printing our message
                }) => MoveOpt::Some(Directions::LEFT),
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                }) => MoveOpt::Some(Directions::RIGHT),
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                }) => MoveOpt::Some(Directions::UP),
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                }) => MoveOpt::Some(Directions::DOWN),
                _ => MoveOpt::Same
            };
        } else {
            return MoveOpt::Same
        }
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}
