pub type Position = [i32; 2];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Shape {
    I,
}

#[derive(Copy, Clone)]
pub struct Tetromino {
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

    pub fn blocks(&self) -> [[i32; 2]; 4] {
        tetromino_blocks(self.shape, self.orientation)
    }
}

fn tetromino_blocks(shape: Shape, orientation: Orientation) -> [[i32; 2]; 4] {
    match shape {
        Shape::I => match orientation {
            Orientation::North => [[-1, 0], [0, 0], [1, 0], [2, 0]],
            Orientation::East => [[1, 1], [1, 0], [1, -1], [1, -2]],
            Orientation::South => [[2, -1], [1, -1], [0, -1], [-1, -1]],
            Orientation::West => [[0, -2], [0, -1], [0, 0], [0, 1]],
        },
    }
}

fn rotate_position_clockwise(position: [i32; 2], origin_scaled_by_two: [i32; 2]) -> [i32; 2] {
    let x = position[0] * 2 - origin_scaled_by_two[0];
    let y = position[1] * 2 - origin_scaled_by_two[1];
    let x_t = (y + origin_scaled_by_two[0]) / 2;
    let y_t = (-x + origin_scaled_by_two[1]) / 2;
    [x_t, y_t]
}

fn rotate_positions_clockwise(
    coordinates: [[i32; 2]; 4],
    origin_scaled_by_two: [i32; 2],
) -> [[i32; 2]; 4] {
    let mut new_coordinates: [[i32; 2]; 4] = coordinates;
    for (i, val) in coordinates.iter().enumerate() {
        new_coordinates[i] = rotate_position_clockwise(*val, origin_scaled_by_two)
    }
    new_coordinates
}

fn origins_scaled_by_two(shape: Shape) -> [i32; 2] {
    match shape {
        Shape::I => [1, -1],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_position_clockwise() {
        assert_eq!(rotate_position_clockwise([1, 1], [0, 0]), [1, -1]);
        assert_eq!(rotate_position_clockwise([1, -1], [0, 0]), [-1, -1]);
        assert_eq!(rotate_position_clockwise([0, 0], [1, -1]), [1, 0]);
        assert_eq!(rotate_position_clockwise([1, 0], [1, 1]), [0, 0]);

        assert_eq!(
            rotate_positions_clockwise([[-1, 0], [0, 0], [1, 0], [2, 0]], [1, -1]),
            [[1, 1], [1, 0], [1, -1], [1, -2]]
        );

        assert_eq!(
            rotate_positions_clockwise([[-1, 0], [0, 0], [0, 1], [1, 1]], [0, 0]),
            [[0, 1], [0, 0], [1, 0], [1, -1]]
        );

        let orientations = [
            Orientation::North,
            Orientation::East,
            Orientation::South,
            Orientation::West,
        ];
        for orientation in orientations {
            assert_eq!(
                rotate_positions_clockwise(
                    tetromino_blocks(Shape::I, orientation),
                    origins_scaled_by_two(Shape::I)
                ),
                tetromino_blocks(Shape::I, rotate_clockwise(orientation)),
            )
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
}
