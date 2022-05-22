pub type Line = [TetrominoType; 10];
pub type Board = [Line; 24];
pub type Position = [i32; 2];

const fn board_width () -> usize { 10 }
const fn board_height () -> usize { 24 }

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TetrominoType
{
    Empty, I, L, T, Z, S, J, O
}

fn empty_line () -> Line { [TetrominoType::Empty; board_width()] }
fn empty_board () -> Board { [empty_line(); board_height()] }

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
    board [3][2] = TetrominoType::I;
    assert_eq! (board [3][2], TetrominoType::I);
}

fn check_array_bounds (index: i32, max: usize) -> bool
{
    if index < 0 { false } else { (index as usize) < max }
}

fn is_empty (board: &Board, position: Position) -> bool
{
    if !check_array_bounds (position[0], board_width()) { false }
    else if !check_array_bounds (position[1], board_height()) { false }
    else {
        let line_index = position[1] as usize;
        let line_pos = position[0] as usize;
        board [line_index] [line_pos] == TetrominoType::Empty
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
    board1[4][1] = TetrominoType::O;
    assert!(!is_empty(&board1, [1, 4]));
}