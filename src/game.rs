use crate::board::{empty_board, Board, Position};
use crate::tetromino::{self, Orientation, Shape, Tetromino};

struct TetrominoOnBoard {
    tetromino: Tetromino,
    position: Position,
}

struct Game {
    board: Board,
    current_tetromino: Option<TetrominoOnBoard>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: empty_board(),
            current_tetromino: None,
        }
    }

    fn spawn(&mut self) {
        self.current_tetromino = Some(spawn(Shape::I));
    }

    fn put_current_tetromino(&mut self) -> bool {
        if let Some(tetromino_on_board) = &self.current_tetromino {
            if self
                .board
                .can_put(tetromino_on_board.position, &tetromino_on_board.tetromino)
            {
                self.board
                    .put(tetromino_on_board.position, &tetromino_on_board.tetromino);
                self.spawn();
                return true;
            } else {
                self.current_tetromino = None;
            }
        }
        false
    }

    fn can_move_down(&self) -> bool {
        self.current_tetromino.as_ref().map_or(false, |t| {
            self.board.can_put(down(t.position), &t.tetromino)
        })
    }

    fn move_down(&mut self) -> bool {
        if let Some(t) = &self.current_tetromino {
            let new_t = TetrominoOnBoard {
                tetromino: t.tetromino,
                position: down(t.position),
            };
            if self.board.can_put(new_t.position, &new_t.tetromino) {
                self.current_tetromino = Some(new_t);
                return true;
            }
        }
        false
    }

    fn can_rotate_clockwise(&self) -> bool {
        if let Some(t) = &self.current_tetromino {
            let new_t = Tetromino::new(t.tetromino.shape).rotate_clockwise();
            return self.board.can_put(t.position, &new_t);
        }
        false
    }
}

fn start_position() -> Position {
    [4, 22]
}

fn spawn(shape: Shape) -> TetrominoOnBoard {
    TetrominoOnBoard {
        tetromino: Tetromino::new(shape),
        position: start_position(),
    }
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
        let mut t = spawn(Shape::I);
        assert_eq!(t.position, start_position());
        for orientation in [
            Orientation::North,
            Orientation::East,
            Orientation::South,
            Orientation::West,
        ] {
            assert_eq!(t.tetromino.orientation, orientation);
            assert!(board.can_put(t.position, &t.tetromino));
            t.tetromino = t.tetromino.rotate_clockwise();
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
        assert!(game.board.can_put(start_position(), &tetromino));
        assert!(!game.can_move_down());
    }

    #[test]
    fn test_move_down() {
        let mut game = Game::new();
        assert!(!game.can_move_down());

        game.spawn();
        assert!(game.move_down());

        game.board.put(
            down(down(start_position())),
            &Tetromino {
                shape: Shape::I,
                orientation: Orientation::North,
            },
        );

        assert!(!game.move_down());
    }

    #[test]
    fn test_put_currrent_tetronimo() {
        let mut game = Game::new();
        assert!(!game.put_current_tetromino());
        game.spawn();
        assert!(game.put_current_tetromino());
        assert!(!game.put_current_tetromino());
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
