use crate::snake::Position;
use crate::Items;
use rand::{prelude::{
    thread_rng,
    ThreadRng,
}, Rng};

use std::ops::{
    Index,
    IndexMut
};

pub const DEFAULT: usize = 16;

/// Contains the entire board, with interactions with it
pub struct Board {
    board: Vec<Vec<Items>>,
    max_x: usize,
    max_y: usize,
    rng: ThreadRng,
}

/// Used to get a board where there are no gaps in the walls
fn board_ngates(size_x: usize, size_y: usize) -> Vec<Vec<Items>>  {
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
    board
}

/// Used to get a board where there are gaps in the walls
fn board_gates(size_x: usize, size_y: usize) -> Vec<Vec<Items>> {
    let mut board = vec![vec![Items::EMPTY; size_x]; size_y];
    for y_pos in 0..board.len() {
        for x_pos in 0..board[y_pos].len() {
            if x_pos == (size_x - 1) || x_pos == 0 {
                if y_pos > ((size_y/2) + 2) || y_pos < ((size_y/2) - 2) {
                    board[y_pos][x_pos] = Items::WALL;
                }
            } else if y_pos == (size_y-1) || y_pos == 0 {
                if x_pos > ((size_x/2) + 2) || x_pos < ((size_x/2) - 2) {
                    board[y_pos][x_pos] = Items::WALL;
                }
            }
        }
    }
    board
}

impl Board {
    /// Creates a new boad, and populates it
    pub fn new(size_x: usize, size_y: usize, gates: bool) -> Self {
        let board = if gates {
            board_gates(size_x, size_y)
        } else {
            board_ngates(size_x, size_y)
        };
        Self {
            board,
            max_x: size_x,
            max_y: size_y,
            rng: thread_rng(),
        }
    }

    pub fn get_max_size(&self) -> (usize, usize) {
        (self.max_x, self.max_y)
    }

    /// Used to set a new fruit on the board
    ///
    /// Checks if a position is empty first
    pub fn fruit(&mut self) -> bool {
        let mut fruit_pos = self.get_rand_block();
        while !self.change_position(&fruit_pos, Items::FRUIT) {
            fruit_pos = self.get_rand_block();
        }
        true
    }

    /// Used to get a random position on the board, inside the walls
    /// Returns that random position as a Position-type
    fn get_rand_block(&mut self) -> Position {
        let x = self.rng.gen_range(1..(self.max_x-2)) as isize;
        let y = self.rng.gen_range(1..(self.max_y-2)) as isize;
        Position::new(x, y)
    } 

    /// Checks if a position is empty
    pub fn check_position(&self, pos: &Position, ident: Items) -> bool {
        self[pos] == ident
    }

    /// Changes a position to another if it is not a wall
    /// Returns true if the position changes, else false
    pub fn change_position(&mut self, pos: &Position, change: Items) -> bool {
        return if !(self.check_position(pos, Items::WALL) || self.check_position(pos, Items::SNAKE)) {
            self[pos] = change;
            true
        } else {
            false
        }
    }

    pub fn remove_position(&mut self, pos: &Position) -> bool {
        return if !self.check_position(pos, Items::WALL){
            self[pos] = Items::EMPTY;
            true
        } else {
            false
        }
    }

    /// Used to get a position that overflows the board
    pub fn get_overflow_pos(&mut self, pos: Position) -> Position {
        return if pos.x == (self.max_x as isize) {
            Position::new(0, pos.y)
        } else if pos.y == (self.max_y as isize) {
            Position::new(pos.x, 0)
        } else if pos.x < 0 {
            Position::new((self.max_x - 1) as isize, pos.y)
        } else if pos.y < 0 {
            Position::new(pos.x, (self.max_y - 1) as isize)
        } else {
            pos
        }
    }

    /// Returns the underlying vectors
    pub fn get_vec(&self) -> &Vec<Vec<Items>> {
        &self.board
    } 
}

impl Default for Board {
    fn default() -> Self {
        Self::new(DEFAULT, DEFAULT, false)
    }
}

impl Index<&Position> for Board {
    type Output = Items;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.board[index.y as usize][index.x as usize]
    }
}

impl IndexMut<&Position> for Board {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.board[index.y as usize][index.x as usize]
    }
}

#[cfg(test)]
mod board_test {

    use crate::board::*;

    fn get_board() -> Board {
        Board::new(8, 8, false)
    }

    #[test]
    fn test_position() {
        let board = get_board();
        assert!(board.check_position(&Position::new(0, 0), Items::WALL));
        assert!(board.check_position(&Position::new(1, 1), Items::EMPTY));
    }

    #[test]
    fn test_change_position() {
        let mut board = get_board();
        let pos = Position::new(1, 1);
        assert!(board.check_position(&pos, Items::EMPTY));
        assert!(board.change_position(&pos, Items::FRUIT));
        assert!(board.check_position(&pos, Items::FRUIT));

        let mut board = get_board();
        let pos = Position::new(0, 0);
        assert!(board.check_position(&pos, Items::WALL));
        assert_eq!(false, board.change_position(&pos, Items::EMPTY));
        assert!(board.check_position(&pos, Items::WALL));

        let mut board = get_board();
        let pos = Position::new(1, 1);
        assert!(board.check_position(&pos, Items::EMPTY));
        assert!(board.change_position(&pos, Items::SNAKE));
        assert!(board.check_position(&pos, Items::SNAKE));
    }

    #[test]
    fn test_add_fruit() {
        let mut board = get_board();
        assert!(board.fruit());
    }
}
