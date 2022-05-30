use crossterm::{
    self, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use std::io::{stdout, Stdout};

use crate::Items;
use crate::{
    board::Board,
    snake::{Position, Snake},
};

const WALL: &str = " W";
const FRUIT: &str = " %";
const SNEK: &str = " S";
const EMPTY: &str = "  ";

pub struct Term {
    stdout: Stdout,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    board_size: (u16, u16),
}

impl Term {
    pub fn new(board_size: (usize, usize)) -> Self {
        let backend = CrosstermBackend::new(stdout());
        let board_width = (board_size.0 * 2 + 3) as u16;
        let board_height = (board_size.1 + 2) as u16;
        let board_size = (board_width, board_height);
        let term = Term {
            stdout: stdout(),
            terminal: Terminal::new(backend).unwrap(),
            board_size,
        };
        enable_raw_mode().unwrap();
        execute!(&term.stdout, EnterAlternateScreen).unwrap();
        term
    }

    pub fn render(
        &mut self,
        board: &Board,
        stats: Vec<&str>,
        players: Vec<&Snake>,
        fruits: &Vec<(Position, Items)>,
    ) {
        self.terminal
            .draw(|f| {
                let board_rect = Rect {
                    x: 0,
                    y: 0,
                    width: self.board_size.0,
                    height: self.board_size.1,
                };
                let stats_rect = Rect {
                    x: self.board_size.0 + 1,
                    y: 0,
                    width: self.board_size.0,
                    height: self.board_size.1,
                };
                print_board(board, players, fruits, f, board_rect);
                print_stats(stats, f, stats_rect);
            })
            .unwrap();
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    }
}

/// Used to print the stats to the screen
fn print_stats<B: tui::backend::Backend>(stats: Vec<&str>, f: &mut Frame<B>, chunk: Rect) {
    let rows: Vec<ListItem> = stats
        .iter()
        .map(|x| ListItem::new(format!("{x}")))
        .collect();
    let text = List::new(rows).block(
        Block::default()
            .title("stats")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    let chunk = Rect::new(chunk.x, chunk.y, (stats[0].len() + 4) as u16, chunk.height);
    f.render_widget(text, chunk);
}

///used to print the board to the screen
///
///board is the board to print
///stdout is used to print
fn print_board<B: tui::backend::Backend>(
    board: &Board,
    players: Vec<&Snake>,
    fruits: &Vec<(Position, Items)>,
    f: &mut Frame<B>,
    chunk: Rect,
) {
    let mut rows = vec![];
    for each in board.get_vec() {
        let mut cell_row = vec![];
        for cell in each {
            cell_row.push(match cell {
                Items::WALL => Span::styled(WALL, Style::default().bg(Color::Gray)),
                _ => Span::from(EMPTY),
            });
        }
        rows.push(cell_row);
    }
    for player in players {
        for pos in player.get_tail() {
            let snake = match player.get_items() {
                Items::SNAKE => (Span::styled(SNEK, Style::default().bg(Color::Green))),
                Items::OSNAKE => (Span::styled(SNEK, Style::default().bg(Color::Yellow))),
                _ => Span::from(EMPTY),
            };
            rows[pos.y as usize][pos.x as usize] = snake;
        }
    }
    for (fruit_pos, fruit_type) in fruits {
        let fruit = match fruit_type {
            Items::FRUIT => Span::styled(FRUIT, Style::default().fg(Color::Red)),
            Items::OFRUIT => Span::styled(FRUIT, Style::default().fg(Color::Blue)),
            _ => Span::from(EMPTY),
        };
        rows[fruit_pos.y as usize][fruit_pos.x as usize] = fruit;
    }
    let mut paragraph = vec![];
    for each in rows {
        paragraph.push(Spans::from(each));
    }
    let text = Paragraph::new(paragraph).block(
        Block::default()
            .title("Snake")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    f.render_widget(text, chunk)
}
