use crate::board::{Board, Position};
use crate::tetromino::{Shape, Tetromino};

fn spawn() -> (Tetromino, Position) {
    (Tetromino::new(Shape::I), [4, 22])
}

#[test]
fn test_spawn() {
    let (_, position) = spawn();
    assert_eq!(position, [4, 22]);
}
