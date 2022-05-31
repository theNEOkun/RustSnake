use crossterm::{
    self, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    text::Spans,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use std::io::{stdout, Stdout};

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

    pub fn render<'a>(
        &mut self,
        board: Vec<Spans<'a>>,
        stats: &Vec<String>,
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
                print_board(board, f, board_rect);
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
fn print_stats<B: tui::backend::Backend>(stats: &Vec<String>, f: &mut Frame<B>, chunk: Rect) {
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
fn print_board<'a, B: tui::backend::Backend>(
    paragraph: Vec<Spans<'a>>,
    f: &mut Frame<B>,
    chunk: Rect,
) {
    let text = Paragraph::new(paragraph.clone()).block(
        Block::default()
            .title("Snake")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    f.render_widget(text, chunk)
}
