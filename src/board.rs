use crate::position::Position;
use crate::tetromino::{Shape, Tetromino};

const fn board_width() -> usize {
    10
}
const fn board_height() -> usize {
    24
}

pub type BoardPosition = [usize; 2];

pub type Line = [BoardContent; board_width()];

pub struct Board {
    lines: [Line; board_height()],
}

impl Board {
    fn height(&self) -> usize {
        self.lines.len()
    }
    fn width(&self) -> usize {
        self.lines[0].len()
    }

    fn get(&self, pos: Position) -> BoardContent {
        match board_position(pos) {
            None => BoardContent::Blocked,
            Some(board_position) => self.lines[board_position[1]][board_position[0]],
        }
    }

    fn is_free(&self, pos: Position) -> bool {
        self.get(pos) == BoardContent::Empty
    }

    fn set(&mut self, pos: Position, tetronimo: Shape) -> bool {
        match board_position(pos) {
            None => false,
            Some(board_position) => {
                self.lines[board_position[1]][board_position[0]] =
                    BoardContent::Tetromino(tetronimo);
                true
            }
        }
    }

    pub fn can_put(&self, tetromino: &Tetromino) -> bool {
        for i in tetromino.blocks().iter().map(|p| *p + tetromino.position) {
            if !self.is_free(i) {
                return false;
            }
        }
        true
    }

    pub fn put(&mut self, tetromino: &Tetromino) {
        tetromino
            .blocks()
            .map(|p| p + tetromino.position)
            .map(|p| self.set(p, tetromino.shape));
    }

    pub fn remove_full_lines(&mut self) -> usize {
        let mut read_index: usize = 0;
        let mut write_index: usize = 0;
        while read_index < board_height() {
            while read_index < board_height() && is_line_full(&self.lines[read_index]) {
                read_index += 1;
            }
            self.lines[write_index] = if read_index < board_height() {
                self.lines[read_index]
            } else {
                empty_line()
            };
            write_index += 1;
            read_index += 1;
        }
        read_index - write_index
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BoardContent {
    Empty,
    Blocked,
    Tetromino(Shape),
}

fn empty_line() -> Line {
    [BoardContent::Empty; board_width()]
}

fn is_line_full(line: &Line) -> bool {
    for content in line.iter() {
        if content == &BoardContent::Empty {
            return false;
        }
    }
    true
}

fn is_line_empty(line: &Line) -> bool {
    for content in line.iter() {
        if content != &BoardContent::Empty {
            return false;
        }
    }
    true
}

pub fn empty_board() -> Board {
    Board {
        lines: [empty_line(); board_height()],
    }
}

fn check_array_bounds(index: i32, max: usize) -> bool {
    if index < 0 {
        false
    } else {
        (index as usize) < max
    }
}

fn board_position(pos: Position) -> Option<BoardPosition> {
    if !check_array_bounds(pos.x, board_width()) {
        None
    } else if !check_array_bounds(pos.y, board_height()) {
        None
    } else {
        Some([pos.x as usize, pos.y as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let board: Board = empty_board();
        let position = Position::new(3, 5);
        assert!(board.is_free(position));

        assert!(!board.is_free(Position::new(-1, 0)));
        assert!(!board.is_free(Position::new(0, -1)));
        assert!(!board.is_free(Position::new(board_width() as i32, 0)));
        assert!(!board.is_free(Position::new(0, board_height() as i32)));

        assert!(board.is_free(Position::new(board_width() as i32 - 1, 0)));
        assert!(board.is_free(Position::new(0, board_height() as i32 - 1)));

        let mut board1 = empty_board();
        board1.set(position, Shape::I);
        assert!(!board1.is_free(position));
    }

    #[test]
    fn test_board_get() {
        let board: Board = empty_board();
        let position = Position::new(3, 5);
        assert_eq!(board.get(position), BoardContent::Empty);

        assert_eq!(board.get(Position::new(-1, 0)), BoardContent::Blocked);
        assert_eq!(board.get(Position::new(0, -1)), BoardContent::Blocked);
        assert_eq!(
            board.get(Position::new(board_width() as i32, 0)),
            BoardContent::Blocked
        );
        assert_eq!(
            board.get(Position::new(0, board_height() as i32)),
            BoardContent::Blocked
        );

        assert_eq!(
            board.get(Position::new(board_width() as i32 - 1, 0)),
            BoardContent::Empty
        );
        assert_eq!(
            board.get(Position::new(0, board_height() as i32 - 1)),
            BoardContent::Empty
        );

        let mut board1 = empty_board();
        board1.set(position, Shape::I);
        assert_eq!(board1.get(position), BoardContent::Tetromino(Shape::I));
    }

    #[test]
    fn test_can_put() {
        let board: Board = empty_board();
        let tetromino = Tetromino::new(Position::new(0, 0), Shape::I);
        assert!(!board.can_put(&tetromino));
        assert!(!board.can_put(&tetromino.get_moved(Position::new(0, -1))));
        assert!(board.can_put(&tetromino.get_moved(Position::new(1, board_height() as i32 - 1))));
    }

    #[test]
    fn test_put() {
        let mut board: Board = empty_board();
        let tetromino = Tetromino::new(Position::new(0, 0), Shape::I);
        board.put(&tetromino);
        assert_eq!(
            board.get(Position::new(0, 0)),
            BoardContent::Tetromino(Shape::I)
        );
        assert_eq!(
            board.get(Position::new(1, 0)),
            BoardContent::Tetromino(Shape::I)
        );
        assert_eq!(
            board.get(Position::new(2, 0)),
            BoardContent::Tetromino(Shape::I)
        );
        assert_eq!(board.get(Position::new(3, 0)), BoardContent::Empty);
        assert_eq!(board.get(Position::new(0, 1)), BoardContent::Empty);
    }

    #[test]
    fn board_size() {
        let board: Board = empty_board();
        assert_eq!(board.height(), board_height());
        assert_eq!(board.width(), board_width());
    }

    #[test]
    fn board_access() {
        let mut board: Board = empty_board();
        let position = Position::new(2, 3);
        let tetromino = Shape::I;
        assert!(board.set(position, tetromino));
        assert!(!board.set(Position::new(-1, 0), tetromino));
        assert_eq!(board.get(position), BoardContent::Tetromino(tetromino));
    }

    #[test]
    fn test_line_full() {
        let board: Board = empty_board();
        assert!(!is_line_full(&board.lines[0]));
        let mut board = board;

        for i in 0..6 {
            board.lines[4][i] = BoardContent::Tetromino(Shape::I);
        }
        assert!(!is_line_full(&board.lines[4]));

        for content in &mut board.lines[4] {
            *content = BoardContent::Tetromino(Shape::I);
        }
        assert!(is_line_full(&board.lines[4]));
    }

    #[test]
    fn test_remove_full_lines() {
        let fill_line_partly = |board: &mut Board, line_number: usize| {
            for j in 0..6 {
                board.lines[line_number][j] = BoardContent::Tetromino(Shape::I);
            }
        };

        let fill_line = |board: &mut Board, line_number: usize| {
            for content in &mut board.lines[line_number] {
                *content = BoardContent::Tetromino(Shape::I);
            }
        };

        let mut board = empty_board();
        assert_eq!(board.remove_full_lines(), 0);

        fill_line(&mut board, 0);
        fill_line_partly(&mut board, 1);
        fill_line(&mut board, 2);
        fill_line(&mut board, 3);
        fill_line_partly(&mut board, 4);
        fill_line_partly(&mut board, 5);
        fill_line(&mut board, 6);
        fill_line_partly(&mut board, 7);

        let num_removed = board.remove_full_lines();
        assert_eq!(num_removed, 4);
        for line in &board.lines {
            assert!(!is_line_full(line));
        }
        assert!(is_line_empty(&board.lines[4]));
    }
}
