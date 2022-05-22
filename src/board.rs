const fn board_width () -> usize { 10 }
const fn board_height () -> usize { 24 }

pub type Position = [i32; 2];
pub type BoardPosition = [usize; 2];

pub type Line = [BoardContent; board_width ()];

pub struct Board {
    lines: [Line; board_height ()],
}    

impl Board {
    fn height (&self) -> usize { self.lines.len () }
    fn width (&self) -> usize { self.lines[0].len () }
    
    fn safe_get (&self, pos: BoardPosition) -> BoardContent { self.lines [pos[1]][pos[0]] }

    fn get (&self, pos: Position) -> BoardContent {
        match board_position (pos) {
            None => BoardContent::Blocked,
            Some (board_position) => self.safe_get (board_position)
        }
    }

    fn is_empty (&self, pos: Position) -> bool { self.get (pos) == BoardContent::Empty }
    
    fn set (&mut self, pos: Position, tetronimo: TetrominoType) -> bool {
        match board_position (pos) {
            None => false,
            Some (board_position) => {
                self.lines [board_position[1]][board_position[0]] = BoardContent::Tetromino(tetronimo);
                true
            }
        }
    }
}    

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TetrominoType
{
    I, L, T, Z, S, J, O
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BoardContent
{
    Empty,
    Blocked,
    Tetromino(TetrominoType),
}

fn empty_line () -> Line { [BoardContent::Empty; board_width()] }
fn empty_board () -> Board { Board { lines: [empty_line(); board_height()] } }

#[test]
fn board_size ()
{
    let board: Board = empty_board ();
    assert_eq!(board.height(), board_height());
    assert_eq!(board.width(), board_width());
}

#[test]
fn board_access() {
    let mut board: Board = empty_board ();
    let position = [2,3];
    let tetromino = TetrominoType::I;
    board.set (position, tetromino);
    assert_eq! (board.get (position), BoardContent::Tetromino(tetromino));
}

fn check_array_bounds (index: i32, max: usize) -> bool
{
    if index < 0 { false } else { (index as usize) < max }
}

fn board_position (pos: Position) -> Option <BoardPosition>
{
    if !check_array_bounds (pos[0], board_width()) { None }
    else if !check_array_bounds (pos[1], board_height()) { None }
    else { Some ([pos[0] as usize, pos[1] as usize]) }
}

#[test]
fn test_is_empty() {
    let board: Board = empty_board ();
    let position = [3,5];
    assert! (board.is_empty (position));
    
    assert! (!board.is_empty ([-1, 0]));
    assert! (!board.is_empty ([0, -1]));
    assert! (!board.is_empty ([board_width() as i32, 0]));
    assert! (!board.is_empty ([0, board_height() as i32]));

    assert! (board.is_empty ([board_width() as i32 - 1, 0]));
    assert! (board.is_empty ([0, board_height() as i32 - 1]));

    let mut board1 = empty_board();
    board1.set (position, TetrominoType::O);
    assert!(!board1.is_empty(position));
}