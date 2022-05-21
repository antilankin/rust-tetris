pub type Line = [TetrominoType; 10];
pub type Board = [Line; 24];

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TetrominoType
{
    Empty, I, L, T, Z, S, J, O
}

fn empty_line () -> Line { [TetrominoType::Empty; 10] }
fn empty_board () -> Board { [empty_line(); 24] }

#[test]
fn board_size ()
{
    let board: Board = empty_board ();
    assert_eq!(board.len(), 24);
}

#[test]
fn line_size ()
{
    let line: Line = empty_line ();
    assert_eq!(line.len(), 10);
}

#[test]
fn line_access() {
    let mut line: Line = empty_line ();
    line [0] = TetrominoType::J;
    assert_eq! (line [0], TetrominoType::J);
}

