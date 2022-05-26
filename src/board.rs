use crate::tetromino::{Shape, Tetromino};

const fn board_width() -> usize {
    10
}
const fn board_height() -> usize {
    24
}

pub type Position = [i32; 2];
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

    pub fn can_put(&self, pos: Position, tetromino: &Tetromino) -> bool {
        for i in tetromino
            .blocks()
            .iter()
            .map(|p| [p[0] + pos[0], p[1] + pos[1]])
        {
            if !self.is_free(i) {
                return false;
            }
        }
        true
    }

    fn put(&mut self, pos: Position, tetromino: &Tetromino) {
        tetromino
            .blocks()
            .map(|p| [p[0] + pos[0], p[1] + pos[1]])
            .map(|p| self.set(p, tetromino.shape));
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

pub fn empty_board() -> Board {
    Board {
        lines: [empty_line(); board_height()],
    }
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
    let position = [2, 3];
    let tetromino = Shape::I;
    assert!(board.set(position, tetromino));
    assert!(!board.set([-1, 0], tetromino));
    assert_eq!(board.get(position), BoardContent::Tetromino(tetromino));
}

fn check_array_bounds(index: i32, max: usize) -> bool {
    if index < 0 {
        false
    } else {
        (index as usize) < max
    }
}

fn board_position(pos: Position) -> Option<BoardPosition> {
    if !check_array_bounds(pos[0], board_width()) {
        None
    } else if !check_array_bounds(pos[1], board_height()) {
        None
    } else {
        Some([pos[0] as usize, pos[1] as usize])
    }
}

#[test]
fn test_is_empty() {
    let board: Board = empty_board();
    let position = [3, 5];
    assert!(board.is_free(position));

    assert!(!board.is_free([-1, 0]));
    assert!(!board.is_free([0, -1]));
    assert!(!board.is_free([board_width() as i32, 0]));
    assert!(!board.is_free([0, board_height() as i32]));

    assert!(board.is_free([board_width() as i32 - 1, 0]));
    assert!(board.is_free([0, board_height() as i32 - 1]));

    let mut board1 = empty_board();
    board1.set(position, Shape::I);
    assert!(!board1.is_free(position));
}

#[test]
fn test_board_get() {
    let board: Board = empty_board();
    let position = [3, 5];
    assert_eq!(board.get(position), BoardContent::Empty);

    assert_eq!(board.get([-1, 0]), BoardContent::Blocked);
    assert_eq!(board.get([0, -1]), BoardContent::Blocked);
    assert_eq!(board.get([board_width() as i32, 0]), BoardContent::Blocked);
    assert_eq!(board.get([0, board_height() as i32]), BoardContent::Blocked);

    assert_eq!(
        board.get([board_width() as i32 - 1, 0]),
        BoardContent::Empty
    );
    assert_eq!(
        board.get([0, board_height() as i32 - 1]),
        BoardContent::Empty
    );

    let mut board1 = empty_board();
    board1.set(position, Shape::I);
    assert_eq!(board1.get(position), BoardContent::Tetromino(Shape::I));
}

#[test]
fn test_can_put() {
    let board: Board = empty_board();
    let tetromino = Tetromino::new(Shape::I);
    assert!(!board.can_put([0, 0], &tetromino));
    assert!(!board.can_put([0, -1], &tetromino));
    assert!(board.can_put([1, board_height() as i32 - 1], &tetromino));
}

#[test]
fn test_put() {
    let mut board: Board = empty_board();
    let tetromino = Tetromino::new(Shape::I);
    board.put([0, 0], &tetromino);
    assert_eq!(board.get([0, 0]), BoardContent::Tetromino(Shape::I));
    assert_eq!(board.get([1, 0]), BoardContent::Tetromino(Shape::I));
    assert_eq!(board.get([2, 0]), BoardContent::Tetromino(Shape::I));
    assert_eq!(board.get([3, 0]), BoardContent::Empty);
    assert_eq!(board.get([0, 1]), BoardContent::Empty);
}
