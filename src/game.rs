use crate::board::{empty_board, Board, Position};
use crate::tetromino::{Orientation, Shape, Tetromino};

fn start_position() -> Position {
    [4, 22]
}

fn spawn(shape: Shape) -> (Tetromino, Position) {
    (Tetromino::new(shape), start_position())
}

fn move_down(board: &Board, tetromino: &Tetromino, position: Position) -> Option<Position> {
    let next_position = [position[0], position[1] - 1];
    if board.can_put(next_position, &tetromino) {
        Some(next_position)
    } else {
        None
    }
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

#[test]
fn test_move_down() {
    let board = empty_board();
    let (tetromino, position) = spawn(Shape::I);
    let next_position = move_down(&board, &tetromino, position);
    assert_eq!(next_position, Some([position[0], position[1] - 1]));
}
