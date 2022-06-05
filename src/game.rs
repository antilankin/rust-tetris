use crate::board::{empty_board, Board, Position};
use crate::tetromino::{Orientation, Shape, Tetromino};

struct TetrominoOnBoard {
    tetromino: Tetromino,
    position: Position,
}

struct Game {
    board: Board,
    current_tetromino: TetrominoOnBoard,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: empty_board(),
            current_tetromino: spawn(Shape::I),
        }
    }

    fn spawn(&mut self) {
        self.current_tetromino = spawn(Shape::I);
    }

    fn put_current_tetromino(&mut self) -> bool {
        if self.board.can_put(
            self.current_tetromino.position,
            &self.current_tetromino.tetromino,
        ) {
            self.board.put(
                self.current_tetromino.position,
                &self.current_tetromino.tetromino,
            );
            self.spawn();
            return true;
        }
        false
    }

    fn can_move_down(&self) -> bool {
        self.board.can_put(
            down(self.current_tetromino.position),
            &self.current_tetromino.tetromino,
        )
    }

    fn move_down(&mut self) -> bool {
        let new_t = TetrominoOnBoard {
            tetromino: self.current_tetromino.tetromino,
            position: down(self.current_tetromino.position),
        };
        if self.board.can_put(new_t.position, &new_t.tetromino) {
            self.current_tetromino = new_t;
            return true;
        }
        false
    }

    fn can_rotate_clockwise(&self) -> bool {
        self.board.can_put(
            self.current_tetromino.position,
            &self.current_tetromino.tetromino.rotate_clockwise(),
        )
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
        let mut game = Game::new();
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
        assert!(game.put_current_tetromino());
        assert!(!game.put_current_tetromino());
    }

    #[test]
    fn test_can_rotate_clockwise() {
        let mut game = Game::new();
        assert!(game.can_rotate_clockwise());

        let tetromino = Tetromino::new(Shape::I);
        game.board.put(down(start_position()), &tetromino);
        assert!(!game.can_rotate_clockwise());
    }
}
