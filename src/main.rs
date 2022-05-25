mod directions;
mod snake;

use crossterm::{
    self,
    execute,
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use snake::Snake;

use std::io::{stdout, Write};

const MAX_SIZE: usize = 16;

fn print(o_str: &str, mut stdout: &std::io::Stdout) {

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(o_str))
        .unwrap();
}

fn main() {
    let mut stdout = stdout();

    print(r#"q to exit"#, &stdout);

    let snake = Snake::new();

    let mut board = vec![vec!["  "; MAX_SIZE]; MAX_SIZE];

    let mut o_string = String::from("\n");

    for (y_pos, each) in board.iter().enumerate() {
        for (x_pos, mut string) in each.iter().enumerate() {
            if x_pos == (MAX_SIZE - 1) || x_pos == 0 {
                string = &&" #";
            } else if y_pos == (MAX_SIZE-1) || y_pos == 0 {
                string = &&" #";
            }
            o_string += string;
        }
        o_string += "\n";
    }
    print(&o_string, &stdout);

    /*
    //key detection
    loop {
        //going to top left corner
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

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
            }) => execute!(stdout, Clear(ClearType::All), Print("Hello world!")).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
            }) => execute!(stdout, Clear(ClearType::All), Print("crossterm is cool")).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
            }) => break,
            _ => (),
        }
    }
    */

}
