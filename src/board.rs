use crate::snake::Position;
use crate::Items;
use rand::{prelude::{
    thread_rng,
    ThreadRng,
}, Rng};

///! Contains the entire board, with interactions with it
pub struct Board {
    board: Vec<Vec<Items>>,
    max_x: usize,
    max_y: usize,
    rng: ThreadRng,
}

impl Board {

    ///Creates a new boad, and populates it
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

    /// Used to set a new fruit on the board
    ///
    /// Checks if a position is empty first
    pub fn fruit(&mut self) {
        let x = self.rng.gen_range(1..(self.max_x-2));
        let y = self.rng.gen_range(1..(self.max_y-2));

        let fruit_pos = Position::new(x, y);
        if self.check_position(&fruit_pos, Items::EMPTY) {
            self.change_position(&fruit_pos, Items::FRUIT);
        } else {
            self.fruit()
        }
    }

    /// Checks if a position is empty
    pub fn check_position(&self, pos: &Position, ident: Items) -> bool {
        self.board[pos.y][pos.x] == ident
    }

    /// Changes a position to another
    pub fn change_position(&mut self, pos: &Position, change: Items) {
        self.board[pos.y][pos.x] = change;
    }

    /// Returns the underlying vector
    pub fn get_vec(&self) -> Vec<Vec<Items>> {
        self.board.clone()
    } 
}

#[cfg(test)]
mod board_test {
    
    use crate::board::*;

    fn get_board() -> Board {
        Board::new(8, 8)
    }

    #[test]
    fn test_position() {
        let board = get_board();
        assert!(board.check_position(&Position::new(0, 0), Items::WALL));
        assert!(board.check_position(&Position::new(1, 1), Items::EMPTY));
    }
}
