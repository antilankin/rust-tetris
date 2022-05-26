mod board;
mod tetromino;

fn spawn() -> (tetromino::Tetromino, board::Position) {
    (tetromino::Tetromino::new(tetromino::Shape::I), [4, 22])
}

#[test]
fn test_spawn() {
    let (_, position) = spawn();
    assert_eq!(position, [4, 22]);
}

fn main() {}
