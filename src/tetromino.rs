use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Add<[i32; 2]> for Position {
    type Output = Position;

    fn add(self, rhs: [i32; 2]) -> Position {
        self + Position::new(rhs[0], rhs[1])
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        self + Position::new(-rhs.x, -rhs.y)
    }
}

impl ops::Sub<[i32; 2]> for Position {
    type Output = Position;

    fn sub(self, rhs: [i32; 2]) -> Position {
        self - Position::new(rhs[0], rhs[1])
    }
}

impl ops::Mul<i32> for Position {
    type Output = Position;

    fn mul(self, rhs: i32) -> Position {
        Position::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Div<i32> for Position {
    type Output = Position;

    fn div(self, rhs: i32) -> Position {
        if rhs == 0 {
            panic!();
        }
        Position::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Shape {
    I,
    O,
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
        for orientation in orientations {
            assert_eq!(
                rotate_positions_clockwise(tetromino_blocks(Shape::I, orientation)),
                tetromino_blocks(Shape::I, rotate_clockwise(orientation)),
            )
        }
    }

    #[test]
    fn test_rotate_tetromino() {
        let north_tetromino = Tetromino::new(Position::new(0, 0), Shape::I);
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
}
