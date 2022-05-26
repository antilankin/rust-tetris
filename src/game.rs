use crate::board::{Position};
use crate::tetromino::{Orientation, Shape, Tetromino};

fn spawn(shape: Shape) -> (Tetromino, Position) {
    (Tetromino::new(shape), [4, 22])
}

#[test]
fn test_spawn() {
    let (tetromino, position) = spawn(Shape::I);
    assert_eq!(position, [4, 22]);
    assert_eq!(tetromino.orientation, Orientation::North);
}
