use crate::position::Position;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Shape {
    I,
    O,
    J,
    L,
    S,
    T,
    Z,
}

fn all_shapes() -> Vec<Shape> {
    vec![
        Shape::I,
        Shape::O,
        Shape::J,
        Shape::L,
        Shape::S,
        Shape::T,
        Shape::Z,
    ]
}

#[derive(Copy, Clone)]
pub struct Tetromino {
    pub position: Position,
    pub shape: Shape,
    pub orientation: Orientation,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Orientation {
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

fn rotate_counterclockwise(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::North => Orientation::West,
        Orientation::East => Orientation::North,
        Orientation::South => Orientation::East,
        Orientation::West => Orientation::South,
    }
}

impl Tetromino {
    pub fn new(position: Position, shape: Shape) -> Self {
        Self {
            position,
            shape,
            orientation: Orientation::North,
        }
    }

    pub fn get_rotated_clockwise(&self) -> Tetromino {
        Tetromino {
            orientation: rotate_clockwise(self.orientation),
            ..*self
        }
    }

    pub fn get_rotated_counterclockwise(&self) -> Tetromino {
        Tetromino {
            orientation: rotate_counterclockwise(self.orientation),
            ..*self
        }
    }

    pub fn get_moved(&self, position: Position) -> Tetromino {
        Tetromino { position, ..*self }
    }

    pub fn get_offset(&self, offset: [i32; 2]) -> Tetromino {
        Tetromino {
            position: self.position + offset,
            ..*self
        }
    }

    pub fn get_moved_down(&self) -> Tetromino {
        self.get_offset([0, -1])
    }

    pub fn get_moved_left(&self) -> Tetromino {
        self.get_offset([-1, 0])
    }

    pub fn get_moved_right(&self) -> Tetromino {
        self.get_offset([1, 0])
    }

    pub fn blocks(&self) -> [Position; 4] {
        tetromino_blocks(self.shape, self.orientation)
    }
}

fn tetromino_blocks(shape: Shape, orientation: Orientation) -> [Position; 4] {
    match shape {
        Shape::I => match orientation {
            Orientation::North => [[-1, 0], [0, 0], [1, 0], [2, 0]],
            Orientation::East => [[0, 1], [0, 0], [0, -1], [0, -2]],
            Orientation::South => [[1, 0], [0, 0], [-1, 0], [-2, 0]],
            Orientation::West => [[0, -1], [0, 0], [0, 1], [0, 2]],
        },
        Shape::O => match orientation {
            Orientation::North => [[0, 0], [1, 0], [1, 1], [0, 1]],
            Orientation::East => [[0, 0], [0, -1], [1, -1], [1, 0]],
            Orientation::South => [[0, 0], [-1, 0], [-1, -1], [0, -1]],
            Orientation::West => [[0, 0], [0, 1], [-1, 1], [-1, 0]],
        },
        Shape::J => match orientation {
            Orientation::North => [[-1, 1], [-1, 0], [0, 0], [1, 0]],
            Orientation::East => [[1, 1], [0, 1], [0, 0], [0, -1]],
            Orientation::South => [[1, -1], [1, 0], [0, 0], [-1, 0]],
            Orientation::West => [[-1, -1], [0, -1], [0, 0], [0, 1]],
        },
        Shape::L => match orientation {
            Orientation::North => [[1, 1], [-1, 0], [0, 0], [1, 0]],
            Orientation::East => [[1, -1], [0, 1], [0, 0], [0, -1]],
            Orientation::South => [[-1, -1], [1, 0], [0, 0], [-1, 0]],
            Orientation::West => [[-1, 1], [0, -1], [0, 0], [0, 1]],
        },
        Shape::S => match orientation {
            Orientation::North => [[1, 1], [-1, 0], [0, 0], [0, 1]],
            Orientation::East => [[1, -1], [0, 1], [0, 0], [1, 0]],
            Orientation::South => [[-1, -1], [1, 0], [0, 0], [0, -1]],
            Orientation::West => [[-1, 1], [0, -1], [0, 0], [-1, 0]],
        },
        Shape::T => match orientation {
            Orientation::North => [[1, 0], [-1, 0], [0, 0], [0, 1]],
            Orientation::East => [[0, -1], [0, 1], [0, 0], [1, 0]],
            Orientation::South => [[-1, 0], [1, 0], [0, 0], [0, -1]],
            Orientation::West => [[0, 1], [0, -1], [0, 0], [-1, 0]],
        },
        Shape::Z => match orientation {
            Orientation::North => [[1, 0], [-1, 1], [0, 0], [0, 1]],
            Orientation::East => [[0, -1], [1, 1], [0, 0], [1, 0]],
            Orientation::South => [[-1, 0], [1, -1], [0, 0], [0, -1]],
            Orientation::West => [[0, 1], [-1, -1], [0, 0], [-1, 0]],
        },
    }
    .map(|[x, y]| Position::new(x, y))
}

fn rotate_position_clockwise(position: Position) -> Position {
    Position::new(position.y, -position.x)
}

fn rotate_positions_clockwise(coordinates: [Position; 4]) -> [Position; 4] {
    coordinates.map(rotate_position_clockwise)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_shapes_is_complete() {
        let mut contains_i = false;
        let mut contains_j = false;
        let mut contains_l = false;
        let mut contains_o = false;
        let mut contains_s = false;
        let mut contains_t = false;
        let mut contains_z = false;
        for shape in all_shapes() {
            match shape {
                Shape::I => contains_i = true,
                Shape::J => contains_j = true,
                Shape::L => contains_l = true,
                Shape::O => contains_o = true,
                Shape::S => contains_s = true,
                Shape::T => contains_t = true,
                Shape::Z => contains_z = true,
            }
        }
        assert!(contains_i);
        assert!(contains_j);
        assert!(contains_o);
        assert!(contains_l);
        assert!(contains_s);
        assert!(contains_t);
        assert!(contains_z);
    }

    fn test_rotated_blocks(shape: Shape, orientation: Orientation) {
        assert_eq!(
            rotate_positions_clockwise(tetromino_blocks(shape, orientation)),
            tetromino_blocks(shape, rotate_clockwise(orientation)),
        );
    }

    #[test]
    fn test_rotate_position_clockwise() {
        assert_eq!(
            rotate_position_clockwise(Position::new(1, 0)),
            Position::new(0, -1)
        );

        assert_eq!(
            rotate_position_clockwise(Position::new(0, 1)),
            Position::new(1, 0)
        );

        let orientations = [
            Orientation::North,
            Orientation::East,
            Orientation::South,
            Orientation::West,
        ];
        for shape in all_shapes() {
            for orientation in orientations {
                test_rotated_blocks(shape, orientation)
            }
        }
    }

    fn test_rotate_tetromino(shape: Shape) {
        let north_tetromino = Tetromino::new(Position::new(0, 0), shape);
        let east_tetromino = north_tetromino.get_rotated_clockwise();
        let south_tetromino = east_tetromino.get_rotated_clockwise();
        let west_tetromino = south_tetromino.get_rotated_clockwise();
        assert_eq!(north_tetromino.orientation, Orientation::North);
        assert_eq!(east_tetromino.orientation, Orientation::East);
        assert_eq!(south_tetromino.orientation, Orientation::South);
        assert_eq!(west_tetromino.orientation, Orientation::West);
        assert_eq!(
            north_tetromino.get_rotated_counterclockwise().orientation,
            Orientation::West
        );
        assert_eq!(
            west_tetromino.get_rotated_counterclockwise().orientation,
            Orientation::South
        );
        assert_eq!(
            south_tetromino.get_rotated_counterclockwise().orientation,
            Orientation::East
        );
        assert_eq!(
            east_tetromino.get_rotated_counterclockwise().orientation,
            Orientation::North
        );
    }

    #[test]
    fn test_rotate_tetrominos() {
        for shape in all_shapes() {
            test_rotate_tetromino(shape);
        }
    }
}
