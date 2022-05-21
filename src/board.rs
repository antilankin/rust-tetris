pub type Line = [TetrominoType; 10];
pub type Board = [Line; 24];

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