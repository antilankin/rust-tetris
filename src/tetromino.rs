#[derive(Copy, Clone)]
enum Shape {
    I,
}

struct Tetromino {
    shape: Shape,
    orientation: Orientation,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Orientation {
    North,
    East,
    South,
    West,
}

fn rotate_clockwise(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::North => Orientation::East,
        Orientation::East => Orientation::South,
        Orientation::South => Orientation::West,
        Orientation::West => Orientation::North,
    }
}

fn blocks(shape: Shape, orientation: Orientation) -> [[i32; 2]; 4] {
    match shape {
        Shape::I => match orientation {
            Orientation::North => [[-1, 0], [0, 0], [1, 0], [2, 0]],
            Orientation::East => [[1, -1], [1, 0], [1, 1], [1, 2]],
            Orientation::South => [[-1, 1], [0, 1], [1, 1], [2, 1]],
            Orientation::West => [[0, -1], [0, 0], [0, 1], [0, 2]],
        },
    }
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            orientation: Orientation::North,
        }
    }

    pub fn rotate_clockwise(&self) -> Tetromino {
        Tetromino {
            shape: self.shape,
            orientation: rotate_clockwise(self.orientation),
        }
    }
}

#[test]
fn test_rotate_tetromino() {
    let north_tetromino = Tetromino::new(Shape::I);
    let east_tetromino = north_tetromino.rotate_clockwise();
    let south_tetromino = east_tetromino.rotate_clockwise();
    let west_tetromino = south_tetromino.rotate_clockwise();
    assert_eq!(north_tetromino.orientation, Orientation::North);
    assert_eq!(east_tetromino.orientation, Orientation::East);
    assert_eq!(south_tetromino.orientation, Orientation::South);
    assert_eq!(west_tetromino.orientation, Orientation::West);
}
