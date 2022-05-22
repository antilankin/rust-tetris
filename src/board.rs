const fn board_width () -> usize { 10 }
const fn board_height () -> usize { 24 }

pub type Line = [TetrominoType; board_width ()];

pub struct Board {
    lines: [Line; board_height ()],
}

impl Board {
    fn len (&self) -> usize { self.lines.len () }
    fn get (&self, line: usize, index: usize) -> TetrominoType { self.lines [line][index] }
    fn set (&mut self, line: usize, index: usize, tetronimo: TetrominoType) { self.lines [line][index] = tetronimo; }
}

pub type Position = [i32; 2];
pub type BoardPosition = [usize; 2];


#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TetrominoType
{
    Empty, Blocked, I, L, T, Z, S, J, O
}

fn empty_line () -> Line { [TetrominoType::Empty; board_width()] }
fn empty_board () -> Board { Board { lines: [empty_line(); board_height()] } }

#[test]
fn board_size ()
{
    let board: Board = empty_board ();
    assert_eq!(board.len(), board_height());
}

#[test]
fn line_size ()
{
    let line: Line = empty_line ();
    assert_eq!(line.len(), board_width());
}

#[test]
fn line_access() {
    let mut line: Line = empty_line ();
    line [0] = TetrominoType::J;
    assert_eq! (line [0], TetrominoType::J);
}

#[test]
fn board_access() {
    let mut board: Board = empty_board ();
    board.set (3, 2, TetrominoType::I);
    assert_eq! (board.get (3, 2), TetrominoType::I);
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

fn content (board: &Board, position: Position) -> TetrominoType
{
    match board_position (position) {
        None => TetrominoType::Blocked,
        Some (board_position) => board.get (board_position[1], board_position[0])
    }
}

fn is_empty (board: &Board, position: Position) -> bool
{
    content (board, position) == TetrominoType::Empty
}

fn set (board: &mut Board, position: Position, tetronimo: TetrominoType)
{
    match board_position (position) {
        Some (safe_position) => board.set (safe_position [1], safe_position [0], tetronimo),
        None => {}
    }
}

#[test]
fn test_is_empty() {
    let board: Board = empty_board ();
    let position = [3,5];
    assert! (is_empty (&board, position));
    
    assert! (!is_empty (&board, [-1, 0]));
    assert! (!is_empty (&board, [0, -1]));
    assert! (!is_empty (&board, [board_width() as i32, 0]));
    assert! (!is_empty (&board, [0, board_height() as i32]));

    assert! (is_empty (&board, [board_width() as i32 - 1, 0]));
    assert! (is_empty (&board, [0, board_height() as i32 - 1]));

    let mut board1 = empty_board();
    set (&mut board1, position, TetrominoType::O);
    assert!(!is_empty(&board1, position));
}