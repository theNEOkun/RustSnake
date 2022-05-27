mod board;
mod snake;
mod terminal;

use board::Board;
use snake::{Directions, Snake};
use terminal::{
    Term,
    MoveOpt
};
use clap::Parser;

use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, Event,poll, read 
};

use std::{thread::sleep, time::{Duration, Instant}};

///!Used to differentiate the different items
#[derive(PartialEq, PartialOrd, Clone)]
pub enum Items {
    WALL = 10,
    EMPTY = 0,
    SNAKE = 1,
    FRUIT = 2,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {    
    #[clap(short, default_value_t=0)]
    x: usize,

    #[clap(short, default_value_t=0)]
    y: usize,

    #[clap(short, long)]
    gaps: bool,
}

fn get_player_one(input: Event) -> MoveOpt<Directions> {
    match input {
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE
        }) => MoveOpt::Some(Directions::LEFT),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE
        }) => MoveOpt::Some(Directions::RIGHT),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE
        }) => MoveOpt::Some(Directions::UP),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE
        }) => MoveOpt::Some(Directions::DOWN),
        _ => MoveOpt::Same
    }
}

fn get_player_two(input: Event) -> MoveOpt<Directions> {
    match input {
        Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE
        }) => MoveOpt::Some(Directions::LEFT),
        Event::Key(KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::NONE
        }) => MoveOpt::Some(Directions::RIGHT),
        Event::Key(KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::NONE
        }) => MoveOpt::Some(Directions::UP),
        Event::Key(KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::NONE
        }) => MoveOpt::Some(Directions::DOWN),
        _ => MoveOpt::Same
    }
}

///Main game loop
///
///param max_size is the size of max x and y
fn gameloop(mut board: Board) {

    let (max_x, max_y) = board.get_max_size();
    let mut snake_1 = Snake::new(
        snake::Position::new((max_x/2) as isize, (max_y/2) as isize),
        get_player_one
    );
    let mut snake_2 = Snake::new(
        snake::Position::new((max_x/2) as isize, (max_y/2) as isize),
        get_player_two
    );
    let mut term = Term::new((max_x, max_y));

    let mut fruit_1 = false;
    let mut fruit_2 = false;

    board.fruit();

    let survival_time = Instant::now();
    loop {
        let curr_pos_1 = snake_1.get_pos();
        board.change_position(&curr_pos_1, Items::SNAKE);

        let curr_pos_2 = snake_2.get_pos();
        board.change_position(&curr_pos_2, Items::SNAKE);

        //going to top left corner
        let secs = survival_time.elapsed().as_secs();
        let mins = secs/60;
        term.render(board.get_vec(), vec![
            &format!("Size of the snake 1: {}", snake_1._get_size()),
            &format!("Fruits eaten 1: {}", snake_1._get_size() - 4),
            &format!("Size of the snake 2: {}", snake_2._get_size()),
            &format!("Fruits eaten 2: {}", snake_2._get_size() - 4),
            &format!("Time elapsed: {}:{}", mins, secs),
        ]);

        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();
            snake_1.mover(event);
            snake_2.mover(event);
        }

        sleep(Duration::from_millis(20));

        match snake_1.move_snake(&mut board) {
            snake::Happen::Some(_) => fruit_1,
            snake::Happen::Break => break,
            _ => false,
        };
        match snake_2.move_snake(&mut board) {
            snake::Happen::Some(_) => fruit_2,
            snake::Happen::Break => break,
            _ => false,
        };
        if let Some(last_pos) = snake_1.get_back() {
            board.remove_position(&last_pos);
        }

        if fruit_1 {
            board.fruit();
            fruit_1 = false;
        }
    }
}

//Main-method
//Takes arguments
fn main() {
    let args = Args::parse();

    if args.x != 0 && args.y != 0 {
        gameloop(Board::new(args.x, args.y, args.gaps));
    }
    else if args.x == 0 && args.y == 0 && args.gaps {
        gameloop(Board::new(board::DEFAULT, board::DEFAULT, args.gaps))
    }
    else {
        gameloop(Board::default())
    }
}
