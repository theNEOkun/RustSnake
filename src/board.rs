use crate::snake::Position;
use crate::Items;
use rand::{prelude::{
    thread_rng,
    ThreadRng,
}, Rng};

/// Contains the entire board, with interactions with it
pub struct Board {
    board: Vec<Vec<Items>>,
    max_x: usize,
    max_y: usize,
    rng: ThreadRng,
}

impl Board {

    /// Creates a new boad, and populates it
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
        let x = self.rng.gen_range(1..(self.max_x-2));
        let y = self.rng.gen_range(1..(self.max_y-2));
        Position::new(x, y)
    } 

    /// Checks if a position is empty
    pub fn check_position(&self, pos: &Position, ident: Items) -> bool {
        self.board[pos.y][pos.x] == ident
    }

    /// Changes a position to another if it is not a wall
    /// Returns true if the position changes, else false
    pub fn change_position(&mut self, pos: &Position, change: Items) -> bool {
        return if !self.check_position(pos, Items::WALL) {
            self.board[pos.y][pos.x] = change;
            true
        } else {
            false
        }
    }

    /// Returns the underlying vectors
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

    #[test]
    fn test_change_position() {
        let mut board = get_board();
        let pos_1 = Position::new(1, 1);
        assert!(board.check_position(&pos_1, Items::EMPTY));
        board.change_position(&pos_1, Items::FRUIT);
        assert!(board.check_position(&pos_1, Items::FRUIT));
        let pos_2 = Position::new(0, 0);
        assert!(board.check_position(&pos_2, Items::WALL));
        board.change_position(&pos_2, Items::EMPTY);
        assert!(board.check_position(&pos_2, Items::WALL));
    }

    #[test]
    fn test_add_fruit() {
        let mut board = get_board();
        assert!(board.fruit());
    }
}
