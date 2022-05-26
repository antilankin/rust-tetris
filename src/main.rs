use tetromino::Tetromino;

use crate::board::Board;
use crate::board::Line;

mod board;
mod tetromino;

fn spawn() -> (tetromino::Tetromino, board::Position) {
    (tetromino::Tetromino::new(tetromino::Shape::I), [4, 22])
}

#[test]
fn test_game() {
    let board = board::empty_board();
    let (tetromino, position) = spawn();
    assert_eq!(position, [4, 22]);
}

fn main() {}
