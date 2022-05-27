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

use std::{thread::sleep, time::{Duration, Instant},process::ExitCode };

///!Used to differentiate the different items
#[derive(PartialEq, PartialOrd, Clone)]
pub enum Items {
    WALL = 10,
    EMPTY = 0,
    SNAKE = 1,
    OSNAKE = 2,
    FRUIT = 3,
    OFRUIT = 4,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {    
    #[clap(short, default_value_t=0)]
    x: usize,

    #[clap(short, default_value_t=0)]
    y: usize,

    #[clap(short, long)]
    multipl: bool,

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

fn gameloop_single(mut board: Board, mut player: Snake) {
    let (max_x, max_y) = board.get_max_size();
    let mut term = Term::new((max_x, max_y));

    board.fruit(player.fruit());

    let survival_time = Instant::now();
    loop {
        let curr_pos = player.get_pos();
        board.change_position(&curr_pos, player.get_items());

        //going to top left corner
        let secs = survival_time.elapsed().as_secs();
        let mins = secs/60;
        term.render(board.get_vec(), vec![
            &format!("Size of the snake: {}", player._get_size()),
            &format!("Fruits eaten: {}", player._get_size() - 4),
            &format!("Time elapsed: {}:{}", mins, secs),
        ]);

        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();
            player.mover(event);
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                }) => break,
                _ => (),
            }
        }

        match player.move_snake(&mut board) {
            snake::Happen::Some(_) => board.fruit(player.fruit()),
            snake::Happen::Break => break,
            _ => false,
        };

        if let Some(last_pos) = player.get_back() {
            board.remove_position(&last_pos);
        }

        sleep(Duration::from_millis(20));
    }
}

///Main game loop
///
///param max_size is the size of max x and y
fn gameloop(mut board: Board, mut player_one: Snake, mut player_two: Snake) {
    let (max_x, max_y) = board.get_max_size();
    let mut term = Term::new((max_x, max_y));

    board.fruit(player_one.fruit());
    board.fruit(player_two.fruit());

    let survival_time = Instant::now();
    loop {
        let curr_pos_1 = player_one.get_pos();
        board.change_position(&curr_pos_1, player_one.get_items());

        let curr_pos_2 = player_two.get_pos();
        board.change_position(&curr_pos_2, player_two.get_items());

        //going to top left corner
        let secs = survival_time.elapsed().as_secs();
        let mins = secs/60;
        term.render(board.get_vec(), vec![
            &format!("Size of the snake 1: {}", player_one._get_size()),
            &format!("Fruits eaten 1: {}", player_one._get_size() - 4),
            &format!("Size of the snake 2: {}", player_two._get_size()),
            &format!("Fruits eaten 2: {}", player_two._get_size() - 4),
            &format!("Time elapsed: {}:{}", mins, secs),
        ]);

        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();
            player_one.mover(event);
            player_two.mover(event);
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                }) => break,
                _ => (),
            }
        }

        match player_one.move_snake(&mut board) {
            snake::Happen::Some(_) => player_one.set_fruit(board.fruit()),
            snake::Happen::Break => break,
            _ => false,
        };

        match player_two.move_snake(&mut board) {
            snake::Happen::Some(_) => player_two.set_fruit(board.fruit()),
            snake::Happen::Break => break,
            _ => false,
        };

        if let Some(last_pos) = player_one.get_back() {
            board.remove_position(&last_pos);
        }
        if let Some(last_pos) = player_two.get_back() {
            board.remove_position(&last_pos);
        }

        sleep(Duration::from_millis(20));
    }
}

//Main-method
//Takes arguments
fn main() {
    let args = Args::parse();

    let (size_x, size_y) = if args.x != 0 && args.y != 0 {
        (args.x, args.y)
    } else {
        (board::DEFAULT, board::DEFAULT)
    };

    if args.multipl {
        gameloop(Board::new(size_x, size_y, args.gaps),
        Snake::new(
            snake::Position::new((size_x/2) as isize,  (size_y/2) as isize),
            Items::SNAKE,
            Items::FRUIT,
            get_player_one
        ),
        Snake::new(
            snake::Position::new((size_x/2) as isize, (size_y/2) as isize),
            Items::OSNAKE,
            Items::OFRUIT,
            get_player_two
        )
        );
    } else {
        gameloop_single(Board::new(size_x, size_y, args.gaps),
        Snake::new(
            snake::Position::new((size_x/2) as isize,  (size_y/2) as isize),
            Items::SNAKE,
            Items::FRUIT,
            get_player_one
        ));
    };
}
