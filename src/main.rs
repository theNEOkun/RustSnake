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

/// Method used to get the opposite direction of a given direction
fn opposite(dirr: &Directions) -> Directions {
    return match dirr {
        Directions::LEFT => Directions::RIGHT,
        Directions::RIGHT => Directions::LEFT,
        Directions::UP => Directions::DOWN,
        Directions::DOWN => Directions::UP,
    };
}

///Main game loop
///
///param max_size is the size of max x and y
fn gameloop(mut board: Board) {

    let (max_x, max_y) = board.get_max_size();
    let mut snake = Snake::new(
        snake::Position::new((max_x/2) as isize, (max_y/2) as isize)
        );
    let mut term = Term::new((max_x, max_y));

    let mut dirr: Directions = Directions::LEFT;
    let mut fruit = false;

    board.fruit();

    let survival_time = Instant::now();
    loop {
        let curr_pos = snake.get_pos();
        board.change_position(&curr_pos, Items::SNAKE);

        //going to top left corner
        let secs = survival_time.elapsed().as_secs();
        let mins = secs/60;
        term.render(board.get_vec(), vec![
            &format!("Size of the snake: {}", snake._get_size()),
            &format!("Fruits eaten: {}", snake._get_size() - 4),
            &format!("Time elapsed: {}:{}", mins, secs),
        ]);

        match term.move_snake() {
            MoveOpt::Some(new_dirr) => {
                dirr = if dirr != opposite(&new_dirr) {
                    new_dirr
                } else {
                    dirr
                }
            }
            MoveOpt::None => break,
            _ => (),
        }

        sleep(Duration::from_millis(20));

        let pos = snake.move_snake(&dirr);
        let pos = board.get_overflow_pos(pos);
        if !board.check_position(&pos, Items::EMPTY) {
            if board.check_position(&pos, Items::FRUIT) {
                fruit = snake.eat();
            } else {
                break;
            }
        }
        if let Some(last_pos) = snake.get_back() {
            board.remove_position(&last_pos);
        }
        snake.set_pos(pos);

        if fruit {
            board.fruit();
            fruit = false;
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
