use crate::board::{empty_board, Board};
use crate::tetromino::{down, Orientation, Position, Shape, Tetromino};

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

    fn spawn(&mut self) -> bool {
        self.current_tetromino = spawn(Shape::I);
        self.board.can_put(&self.current_tetromino)
    }

    fn put_current_tetromino(&mut self) -> bool {
        if self.board.can_put(&self.current_tetromino) {
            self.board.put(&self.current_tetromino);
            self.spawn();
            return true;
        }
        false
    }

    fn move_down(&mut self) -> bool {
        let new_t = self.current_tetromino.move_down();
        if self.board.can_put(&new_t) {
            self.current_tetromino = new_t;
            return true;
        }
        false
    }

    fn rotate_clockwise(&mut self) -> bool {
        let cur_t = &self.current_tetromino;
        let new_t = cur_t.rotate_clockwise();

        for offset in offsets(cur_t.shape, cur_t.orientation, new_t.orientation) {
            let test_t = new_t.get_offset(offset);
            if self.board.can_put(&test_t) {
                self.current_tetromino = test_t;
                return true;
            }
        }
        false
    }
}

fn start_position() -> Position {
    Position::new(4, 22)
}

fn spawn(shape: Shape) -> Tetromino {
    Tetromino::new(start_position(), shape)
}

fn offsets(shape: Shape, from: Orientation, to: Orientation) -> Vec<[i32; 2]> {
    let from_offsets = offset_table(shape, from);
    let to_offsets = offset_table(shape, to);
    from_offsets
        .iter()
        .zip(to_offsets.iter())
        .map(|off| [off.0[0] - off.1[0], off.0[1] - off.1[1]])
        .collect()
}

fn offset_table(shape: Shape, orientation: Orientation) -> Vec<[i32; 2]> {
    match shape {
        Shape::I => offset_table_i(orientation),
    }
}

fn offset_table_i(orientation: Orientation) -> Vec<[i32; 2]> {
    match orientation {
        Orientation::North => vec![[0, 0], [-1, 0], [2, 0], [-1, 0], [2, 0]],
        Orientation::East => vec![[-1, 0], [0, 0], [0, 0], [0, 1], [0, -2]],
        Orientation::South => vec![[-1, 1], [1, 1], [-2, 1], [1, 0], [-2, 0]],
        Orientation::West => vec![[0, 1], [0, 1], [0, 1], [0, -1], [0, 2]],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position() {
        let mut game = Game::new();
        assert_eq!(game.current_tetromino.position, start_position());
        for orientation in [
            Orientation::North,
            Orientation::East,
            Orientation::South,
            Orientation::West,
        ] {
            assert_eq!(game.current_tetromino.orientation, orientation);
            assert!(game.board.can_put(&game.current_tetromino));
            game.rotate_clockwise();
        }
    }

    #[test]
    fn test_spawn() {
        let mut game = Game::new();
        assert!(game.spawn());
        game.put_current_tetromino();
        assert!(!game.spawn());
    }

    #[test]
    fn test_put_currrent_tetronimo() {
        let mut game = Game::new();
        assert!(game.put_current_tetromino());
        assert!(!game.put_current_tetromino());
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
    fn test_rotate_clockwise() {
        let mut game = Game::new();
        game.spawn();
        assert!(game.rotate_clockwise());
        println!("{:?}", game.current_tetromino.position);
        assert!(game.current_tetromino.position == start_position() + [1, 0])
    }
}
