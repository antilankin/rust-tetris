use crate::board::{empty_board, Board};
use crate::tetromino::{Orientation, Position, Shape, Tetromino};

struct Game {
    board: Board,
    current_tetromino: Tetromino,
    lines_removed: usize,
}

enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: empty_board(),
            current_tetromino: spawn(Shape::I),
            lines_removed: 0,
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

    fn move_down(&mut self) -> bool {
        self.update_tetromino(vec![self.current_tetromino.get_moved_down()])
    }

    fn move_left(&mut self) -> bool {
        self.update_tetromino(vec![self.current_tetromino.get_moved_left()])
    }

    fn move_right(&mut self) -> bool {
        self.update_tetromino(vec![self.current_tetromino.get_moved_right()])
    }

    fn drop(&mut self) {
        while self.move_down() {}
    }

    fn tick(&mut self) -> bool {
        if !self.move_down() {
            if !self.board.can_put(&self.current_tetromino) {
                return false;
            }
            self.board.put(&self.current_tetromino);
            self.lines_removed += self.board.remove_full_lines();
            self.spawn();
        }
        true
    }

    fn rotate_clockwise(&mut self) -> bool {
        self.update_tetromino(rotation_candidates(
            self.current_tetromino,
            Direction::Clockwise,
        ))
    }

    fn rotate_counterclockwise(&mut self) -> bool {
        self.update_tetromino(rotation_candidates(
            self.current_tetromino,
            Direction::CounterClockwise,
        ))
    }

    fn update_tetromino(&mut self, candidates: Vec<Tetromino>) -> bool {
        if let Some(t) = self.test_candidates(candidates) {
            self.current_tetromino = t;
            return true;
        }
        false
    }

    fn test_candidates(&self, candidates: Vec<Tetromino>) -> Option<Tetromino> {
        for candidate in candidates {
            if self.board.can_put(&candidate) {
                return Some(candidate);
            }
        }
        None
    }

    fn lines_removed(&self) -> usize {
        self.lines_removed
    }
}

fn start_position() -> Position {
    Position::new(4, 22)
}

fn spawn(shape: Shape) -> Tetromino {
    Tetromino::new(start_position(), shape)
}

fn rotation_candidates(t: Tetromino, direction: Direction) -> Vec<Tetromino> {
    let new_t = match direction {
        Direction::Clockwise => t.get_rotated_clockwise(),
        Direction::CounterClockwise => t.get_rotated_counterclockwise(),
    };
    offsets(t.shape, t.orientation, new_t.orientation)
        .iter()
        .map(|off| new_t.get_offset(*off))
        .collect()
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

    fn drop_left(game: &mut Game) -> usize {
        let mut move_cnt: usize = 0;
        while game.move_left() {
            move_cnt += 1;
        }
        move_cnt
    }

    fn drop_right(game: &mut Game) -> usize {
        let mut move_cnt: usize = 0;
        while game.move_right() {
            move_cnt += 1;
        }
        move_cnt
    }

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
            assert!(game.rotate_clockwise());
        }
    }

    #[test]
    fn test_spawn() {
        let mut game = Game::new();
        game.spawn();
        assert!(game.current_tetromino.position == start_position());
        assert!(game.current_tetromino.orientation == Orientation::North);
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
    fn test_move_left() {
        let mut game = Game::new();
        game.spawn();
        assert_eq!(drop_left(&mut game), 3);
    }

    #[test]
    fn test_move_right() {
        let mut game = Game::new();
        game.spawn();
        assert_eq!(drop_right(&mut game), 3);
    }

    #[test]
    fn test_rotate_clockwise() {
        let mut game = Game::new();
        game.spawn();
        let expected_orientation = game.current_tetromino.get_rotated_clockwise().orientation;
        assert!(game.rotate_clockwise());
        assert_eq!(game.current_tetromino.position, start_position() + [1, 0]);
        assert_eq!(game.current_tetromino.orientation, expected_orientation);
    }

    #[test]
    fn test_wallkick_i() {
        let mut game = Game::new();
        game.spawn();
        game.rotate_clockwise();
        drop_left(&mut game);
        assert_eq!(game.current_tetromino.position, start_position() - [4, 0]);
        game.rotate_counterclockwise();
        assert_eq!(game.current_tetromino.position, start_position() - [3, 0]);
        assert_eq!(game.current_tetromino.orientation, Orientation::North);
    }

    #[test]
    fn test_rotate_counterclockwise() {
        let mut game = Game::new();
        game.spawn();
        let expected_orientation = game
            .current_tetromino
            .get_rotated_counterclockwise()
            .orientation;
        assert!(game.rotate_counterclockwise());
        println!("{:?}", game.current_tetromino.position);
        assert_eq!(game.current_tetromino.position, start_position() + [0, -1]);
        assert_eq!(game.current_tetromino.orientation, expected_orientation);
    }

    #[test]
    fn test_drop() {
        let mut game = Game::new();
        game.spawn();
        game.drop();
        assert_eq!(game.current_tetromino.position, Position::new(4, 0));
    }

    #[test]
    fn test_tick() {
        let mut game = Game::new();
        game.spawn();
        assert!(game.tick());
        assert_eq!(game.current_tetromino.position, start_position() - [0, 1]);
        game.drop();
        assert!(game.tick());
        assert_eq!(game.current_tetromino.position, start_position());
        game.move_down();
        game.put_current_tetromino();
        game.spawn();
        assert!(game.tick());
        assert!(!game.tick());
    }

    #[test]
    fn test_play_game_fill_one_line() {
        let mut game = Game::new();
        game.spawn();
        drop_left(&mut game);
        game.drop();
        game.tick();
        drop_right(&mut game);
        game.drop();
        game.tick();
        game.rotate_clockwise();
        game.drop();
        game.tick();
        game.rotate_counterclockwise();
        game.drop();
        game.tick();
        assert_eq!(game.lines_removed(), 1);
    }
}
