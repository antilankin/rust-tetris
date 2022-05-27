use crate::board::{empty_board, Board, Position};
use crate::tetromino::{Orientation, Shape, Tetromino};

struct Game {
    board: Board,
    current_tetromino: Option<(Tetromino, Position)>,
}

fn new_game() -> Game {
    Game {
        board: empty_board(),
        current_tetromino: None,
    }
}

fn current_tetromino_position(game: &Game) -> Option<Position> {
    game.current_tetromino.as_ref().map(|&(_, p)| p)
}

fn start_position() -> Position {
    [4, 22]
}

fn spawn(shape: Shape) -> (Tetromino, Position) {
    (Tetromino::new(shape), start_position())
}

fn down(position: Position) -> Position {
    [position[0], position[1] - 1]
}

fn move_down(game: &Game) -> Option<Position> {
    current_tetromino_position(game)
        .map(down)
        .and_then(|next_position| {
            if game
                .board
                .can_put(next_position, &game.current_tetromino.as_ref().unwrap().0)
            {
                Some(next_position)
            } else {
                None
            }
        })
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
    let game = new_game();
    let next_position = move_down(&game);
    assert_eq!(next_position, None);

    let mut game = game;
    game.current_tetromino = Some(spawn(Shape::I));
    let next_position = move_down(&game);
    let expected_position = current_tetromino_position(&game).map(down);
    assert_ne!(next_position, None);
    assert_eq!(next_position, expected_position);
}
