use crate::board::{empty_board, Position};
use crate::tetromino::{Orientation, Shape, Tetromino};

fn start_position() -> Position {
    [4, 22]
}

fn spawn(shape: Shape) -> (Tetromino, Position) {
    (Tetromino::new(shape), start_position())
}

#[test]
fn test_spawn() {
    let board = empty_board();
    let (mut tetromino, position) = spawn(Shape::I);
    assert_eq!(position, start_position());
    for orientation in [
        Orientation::North,
        Orientation::East,
        Orientation::South,
        Orientation::West,
    ] {
        assert_eq!(tetromino.orientation, orientation);
        assert!(board.can_put(position, &tetromino));
        tetromino = tetromino.rotate_clockwise();
    }
}
