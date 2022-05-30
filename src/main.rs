mod board;
mod snake;
mod terminal;
mod controller;

use board::Board;
use clap::Parser;
use rand::{
    prelude::{thread_rng, ThreadRng},
    Rng,
};
use snake::{Position, Snake};
use terminal::Term;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

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
    #[clap(short, default_value_t = 0)]
    x: usize,

    #[clap(short, default_value_t = 0)]
    y: usize,

    #[clap(short, long)]
    multipl: bool,

    #[clap(short, long)]
    gaps: bool,

    #[clap(short, long)]
    share_fruit: bool,
}

/// Used to set a new fruit on the board
///
/// Checks if a position is empty first
pub fn fruit(board: &mut Board, fruit: &Items, fruits: &mut Vec<(Position, Items)>) {
    let (max_x, max_y) = board.get_max_size();
    let mut fruit_pos = get_rand_block(max_x, max_y);
    while !board.check_position(&fruit_pos, &Items::EMPTY) {
        fruit_pos = get_rand_block(max_x, max_y);
    }
    board[&fruit_pos] = fruit.clone();
    fruits.push((fruit_pos, fruit.clone()));
}

/// Used to get a random position on the board, inside the walls
/// Returns that random position as a Position-type
fn get_rand_block(max_x: usize, max_y: usize) -> Position {
    let mut RNG: ThreadRng = thread_rng();
    let x = RNG.gen_range(1..(max_x - 2)) as isize;
    let y = RNG.gen_range(1..(max_y - 2)) as isize;
    Position::new(x, y)
}

fn gameloop_single(mut board: Board, mut player: Snake) {
    let (max_x, max_y) = board.get_max_size();
    let mut term = Term::new((max_x, max_y));

    let mut fruits = vec![];
    fruit(&mut board, player.fruit(), &mut fruits);

    let survival_time = Instant::now();
    loop {
        let curr_pos = player.get_pos();
        board.change_position(&curr_pos, player.get_items());

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

        match player.move_snake(&mut board, &mut fruits) {
            snake::Happen::Some(_) => fruit(&mut board, player.fruit(), &mut fruits),
            snake::Happen::Break => break,
            _ => {}
        };

        if let Some(last_pos) = player.get_back() {
            board.remove_position(&last_pos);
        }

        //going to top left corner
        let secs = survival_time.elapsed().as_secs();
        let mins = secs / 60;
        let mut p_info = player.get_info();
        p_info.push(format!("Time elapsed: {}:{}", mins, secs));
        term.render(
            board.get_vec(),
            &p_info,
            &vec![(player.get_tail().clone(), player.get_span())],
            &fruits,
        );

        sleep(Duration::from_millis(20));
    }
}

///Main game loop
///
///param max_size is the size of max x and y
fn gameloop(mut board: Board, mut player_one: Snake, mut player_two: Snake, share: bool) {
    let (max_x, max_y) = board.get_max_size();
    let mut term = Term::new((max_x, max_y));

    let survival_time = Instant::now();

    let mut fruits = vec![];
    if !share {
        fruit(&mut board, player_one.fruit(), &mut fruits);
        fruit(&mut board, player_two.fruit(), &mut fruits);
    } else {
        fruit(&mut board, player_two.fruit(), &mut fruits);
    }
    loop {
        let curr_pos_1 = player_one.get_pos();
        board.change_position(&curr_pos_1, player_one.get_items());

        let curr_pos_2 = player_two.get_pos();
        board.change_position(&curr_pos_2, player_two.get_items());

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

        match player_one.move_snake(&mut board, &mut fruits) {
            snake::Happen::Some(_) => fruit(&mut board, player_one.fruit(), &mut fruits),
            snake::Happen::Break => break,
            _ => (),
        };

        match player_two.move_snake(&mut board, &mut fruits) {
            snake::Happen::Some(_) => fruit(&mut board, player_two.fruit(), &mut fruits),
            snake::Happen::Break => break,
            _ => (),
        };

        if let Some(last_pos) = player_one.get_back() {
            board.remove_position(&last_pos);
        }
        if let Some(last_pos) = player_two.get_back() {
            board.remove_position(&last_pos);
        }

        //going to top left corner
        let secs = survival_time.elapsed().as_secs();
        let mins = secs / 60;
        let mut p_info = player_one.get_info();
        p_info.append(&mut player_two.get_info());
        p_info.push(format!("Time elapsed: {}:{}", mins, secs));
        term.render(
            board.get_vec(),
            &p_info,
            &vec![(player_one.get_tail().clone(), player_one.get_span()), (player_two.get_tail().clone(), player_two.get_span())],
            &fruits,
        );

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
        if args.share_fruit {
            gameloop(
                Board::new(size_x, size_y, args.gaps),
                Snake::new(
                    snake::Position::new((size_x / 2) as isize, (size_y / 2) as isize),
                    Items::SNAKE,
                    Items::FRUIT,
                    controller::get_player_one,
                ),
                Snake::new(
                    snake::Position::new((size_x / 2) as isize, (size_y / 2) as isize),
                    Items::OSNAKE,
                    Items::FRUIT,
                    controller::get_player_two,
                ),
                args.share_fruit
            );
        } else {
            gameloop(
                Board::new(size_x, size_y, args.gaps),
                Snake::new(
                    snake::Position::new((size_x / 2) as isize, (size_y / 2) as isize),
                    Items::SNAKE,
                    Items::FRUIT,
                    controller::get_player_one,
                ),
                Snake::new(
                    snake::Position::new((size_x / 2) as isize, (size_y / 2) as isize),
                    Items::OSNAKE,
                    Items::OFRUIT,
                    controller::get_player_two,
                ),
                args.share_fruit
            );
        }
    } else {
        gameloop_single(
            Board::new(size_x, size_y, args.gaps),
            Snake::new(
                snake::Position::new((size_x / 2) as isize, (size_y / 2) as isize),
                Items::SNAKE,
                Items::FRUIT,
                controller::get_player_one,
            ),
        );
    };
}
