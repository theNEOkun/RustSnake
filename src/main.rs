mod board;
mod snake;
mod terminal;

use board::Board;
use snake::{Directions, Snake};
use terminal::{
    Term,
    MoveOpt
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
    let mut snake = Snake::new(max_x, max_y);
    let mut term = Term::new();

    let mut dirr: Directions = Directions::LEFT;
    let mut fruit = false;

    board.fruit();

    let surivival_time = Instant::now();
    loop {
        let curr_pos = snake.get_pos();
        board.change_position(&curr_pos, Items::SNAKE);

        //going to top left corner
        let size = snake._get_size();
        let eaten = size - 4;
        let time = surivival_time.elapsed().as_secs();
        term.render(board.get_vec(), vec![
            &format!("Size of the snake: {size}"),
            &format!("Number of fruits eaten: {eaten}"),
            &format!("Time elapsed: {time}"),
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
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match &args[1][..] {
            "--size" => {
                let size: usize = (&args[2][..]).parse().unwrap();
                gameloop(Board::new(size, size));
            }
            _ => gameloop(Board::default()),
        }
    } else {
        gameloop(Board::default())
    }
}
