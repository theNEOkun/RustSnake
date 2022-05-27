use crossterm::{
    self,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::Rect ,
    style::{Color, Style},
    widgets::{Block, Paragraph},
    text::{Span, Spans},
    Terminal,
};

use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crate::Directions;
use crate::Items;

const WALL: &str = " W";
const FRUIT: &str = " %";
const SNEK: &str = " S";
const EMPTY: &str = "  ";

pub enum MoveOpt<T> {
    Some(T),
    Same,
    None,
}

pub struct Term {
    stdout: Stdout,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    rect: Rect,
}

impl Term {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let backend = CrosstermBackend::new(stdout());
        let term = Term {
            stdout: stdout(),
            terminal: Terminal::new(backend).unwrap(),
            rect: Rect {
                x: 0,
                y: 0,
                width: (size_x * 2) as u16,
                height: (size_y + 2) as u16
            }
        };
        enable_raw_mode().unwrap();
        execute!(&term.stdout, EnterAlternateScreen).unwrap();
        term
    }

    ///used to print the board to the screen
    ///
    ///board is the board to print
    ///stdout is used to print
    pub fn print_board(&mut self, matrix: &Vec<Vec<Items>>) {
        let mut rows = vec![];
        for each in matrix {
            let mut cell_row = vec![];
            for cell in each {
                cell_row.push(match cell {
                    Items::WALL => Span::styled(WALL, Style::default().bg(Color::Black)),
                    Items::FRUIT => Span::styled(FRUIT, Style::default().bg(Color::Red)),
                    Items::SNAKE => Span::styled(SNEK, Style::default().bg(Color::Green)),
                    _ => Span::from(EMPTY),
                });
            }
            rows.push(Spans::from(cell_row));
        }
        self.terminal
            .draw(|f| {
                let table = Paragraph::new(rows).block(Block::default().title("Snake"));
                f.render_widget(table, self.rect);
            })
            .unwrap();
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
                _ => MoveOpt::Same,
            };
        } else {
            return MoveOpt::Same;
        }
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    }
}
