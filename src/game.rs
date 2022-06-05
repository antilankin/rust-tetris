use crate::board::{empty_board, Board};
use crate::tetromino::{Orientation, Position, Shape, Tetromino};

struct Game {
    board: Board,
    current_tetromino: Tetromino,
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
        if self.board.can_put(&self.current_tetromino) {
            self.board.put(&self.current_tetromino);
            self.spawn();
            return true;
        }
        false
    }

    fn can_move_down(&self) -> bool {
        self.board.can_put(
            &self
                .current_tetromino
                .get_moved(down(self.current_tetromino.position)),
        )
    }

    fn move_down(&mut self) -> bool {
        let new_t = self
            .current_tetromino
            .get_moved(down(self.current_tetromino.position));
        if self.board.can_put(&new_t) {
            self.current_tetromino = new_t;
            return true;
        }
        false
    }

    fn can_rotate_clockwise(&self) -> bool {
        self.board
            .can_put(&self.current_tetromino.rotate_clockwise())
    }
}

fn start_position() -> Position {
    [4, 22]
}

fn spawn(shape: Shape) -> Tetromino {
    Tetromino::new(start_position(), shape)
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
            assert_eq!(t.orientation, orientation);
            assert!(board.can_put(&t));
            t = t.rotate_clockwise();
        }
    }

    #[test]
    fn test_can_move_down() {
        let mut game = Game::new();
        game.spawn();
        assert!(game.can_move_down());

        let tetromino = Tetromino::new(start_position(), Shape::I);
        game.board.put(&tetromino.get_moved(down(start_position())));
        assert!(game.board.can_put(&tetromino));
        assert!(!game.can_move_down());
    }

    #[test]
    fn test_move_down() {
        let mut game = Game::new();
        game.spawn();
        assert!(game.move_down());
        game.put_current_tetromino();
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

        let tetromino = Tetromino::new(start_position(), Shape::I);
        game.board.put(&tetromino.get_moved(down(start_position())));
        assert!(!game.can_rotate_clockwise());
    }
}
