pub type Line = [i32; 10];
pub type Board = [Line; 24];

#[test]
fn board_size ()
{
    let board: Board = [[0; 10]; 24];
    assert_eq!(board.len(), 24);
}

#[test]
fn line_size ()
{
    let line: Line = [0; 10];
    assert_eq!(line.len(), 10);
}

#[test]
fn line_access() {
    let mut line: Line = [0;10];
    line [0] = 1;
    assert_eq! (line [0], 1);
}

