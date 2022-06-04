use crate::board::{empty_board, Board, Position};
use crate::tetromino::{Orientation, Shape, Tetromino};

struct Game {
    board: Board,
    current_tetromino: Option<(Tetromino, Position)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: empty_board(),
            current_tetromino: None,
        }
    }

    pub fn current_position(&self) -> Option<Position> {
        self.current_tetromino.and_then(|c| Some(c.1))
    }

    pub fn spawn(&mut self) {
        assert!(self.current_tetromino.is_none());
        self.current_tetromino = Some(spawn(Shape::I));
    }

    fn can_put(&self, position: Position) -> bool {
        if let Some((tetromino, _)) = self.current_tetromino {
            self.board.can_put(position, &tetromino)
        } else {
            false
        }
    }
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

fn try_move_down(game: &Game) -> Option<Position> {
    game.current_position().map(down).and_then(|next_position| {
        if game.can_put(next_position) {
            Some(next_position)
        } else {
            None
        }
    })
}

fn move_down(game: &mut Game) {
    if let Some(pos) = try_move_down(game) {
        match game.current_tetromino {
            Some((t, _)) => game.current_tetromino = Some((t, pos)),
            None => {}
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_try_move_down() {
        let game = Game::new();
        let next_position = try_move_down(&game);
        assert_eq!(next_position, None);

        let mut game = game;
        game.spawn();
        let next_position = try_move_down(&game);
        let expected_position = game.current_position().map(down);
        assert_ne!(next_position, None);
        assert_eq!(next_position, expected_position);
    }

    #[test]
    fn test_move_down() {
        let mut game = Game::new();
        game.spawn();
        let expected_position = game.current_position().map(down);
        move_down(&mut game);
        assert_eq!(game.current_position(), expected_position);
    }

    #[test]
    fn test_try_put() {
        let game = Game::new();
        assert!(!game.can_put(start_position()));
        let mut game = game;
        game.spawn();
        assert!(game.can_put(start_position()));
        assert!(!game.can_put([0, -4]));
    }
}
