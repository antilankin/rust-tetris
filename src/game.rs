use crate::board::{empty_board, Board, Position};
use crate::tetromino::{self, Orientation, Shape, Tetromino};

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

    fn current_position(&self) -> Option<Position> {
        self.current_tetromino.and_then(|c| Some(c.1))
    }

    fn spawn(&mut self) {
        self.current_tetromino = Some(spawn(Shape::I));
    }

    fn can_put(&self, position: Position) -> bool {
        if let Some((tetromino, _)) = self.current_tetromino {
            self.board.can_put(position, &tetromino)
        } else {
            false
        }
    }

    fn put_current_tetronimo(&mut self) -> bool {
        if let Some((tetromino, position)) = self.current_tetromino {
            if self.board.can_put(position, &tetromino) {
                self.board.put(position, &tetromino);
                self.spawn();
                return true;
            } else {
                self.current_tetromino = None;
            }
        }
        false
    }

    fn can_move_down(&self) -> bool {
        self.current_position()
            .map_or(false, |position| self.can_put(down(position)))
    }

    fn can_rotate_clockwise(&self) -> bool {
        if let Some((mut tetromino, position)) = self.current_tetromino {
            tetromino = tetromino.rotate_clockwise();
            return self.board.can_put(position, &tetromino);
        }
        false
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
    fn test_can_move_down() {
        let game = Game::new();
        assert!(!game.can_move_down());

        let mut game = game;
        game.spawn();
        assert!(game.can_move_down());

        let tetromino = Tetromino::new(Shape::I);
        game.board.put(down(start_position()), &tetromino);
        assert!(game.can_put(start_position()));
        assert!(!game.can_move_down());
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

    #[test]
    fn test_put_currrent_tetronimo() {
        let mut game = Game::new();
        assert!(!game.put_current_tetronimo());
        game.spawn();
        assert!(game.put_current_tetronimo());
        assert!(!game.put_current_tetronimo());
    }

    #[test]
    fn test_can_rotate_clockwise() {
        let game = Game::new();
        assert!(!game.can_rotate_clockwise());

        let mut game = game;
        game.spawn();
        assert!(game.can_rotate_clockwise());

        let tetromino = Tetromino::new(Shape::I);
        game.board.put(down(start_position()), &tetromino);
        assert!(!game.can_rotate_clockwise());
    }
}
