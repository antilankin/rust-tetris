use crate::board::{Position, empty_board};
use crate::tetromino::{Orientation, Shape, Tetromino};

fn spawn(shape: Shape) -> (Tetromino, Position) {
    (Tetromino::new(shape), [4, 22])
}

#[test]
fn test_spawn() {
    let board = empty_board();
    let (tetromino, position) = spawn(Shape::I);
    assert_eq!(position, [4, 22]);
    assert_eq!(tetromino.orientation, Orientation::North);
    assert! (board.can_put(position, &tetromino));
}
