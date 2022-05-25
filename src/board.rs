use crate::snake::Position;
use crate::Items;
use rand::{prelude::{
    thread_rng,
    ThreadRng,
}, Rng};

pub struct Board {
    board: Vec<Vec<Items>>,
    max_x: usize,
    max_y: usize,
    rng: ThreadRng,
}

impl Board {
    pub fn new(size_x: usize, size_y: usize) -> Self {

        let mut board = vec![vec![Items::EMPTY; size_x]; size_y];
        for y_pos in 0..board.len() {
            for x_pos in 0..board[y_pos].len() {
                if x_pos == (size_x - 1) || x_pos == 0 {
                    board[y_pos][x_pos] = Items::WALL;
                } else if y_pos == (size_y-1) || y_pos == 0 {
                    board[y_pos][x_pos] = Items::WALL;
                }

            } 
        }
        Board {
            board,
            max_x: size_x,
            max_y: size_y,
            rng: thread_rng(),
        }
    }

    pub fn fruit(&mut self) {
        let x = self.rng.gen_range(1..(self.max_x-2));
        let y = self.rng.gen_range(1..(self.max_y-2));

        self.change_position(&Position::new(x, y), Items::FRUIT);
    }

    pub fn check_position(&self, pos: &Position, ident: Items) -> bool {
        self.board[pos.y][pos.x] == ident
    }

    pub fn change_position(&mut self, pos: &Position, change: Items) {
        self.board[pos.y][pos.x] = change;
    }

    pub fn get_vec(&self) -> Vec<Vec<Items>> {
        self.board.clone()
    } 
}
