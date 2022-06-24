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
            panic!("Division by zero");
        }
        Position::new(self.x / rhs, self.y / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let p = Position::new(2, -1);
        assert_eq!(p.x, 2);
        assert_eq!(p.y, -1);
    }

    #[test]
    fn test_add() {
        let a = Position::new(2, -1);
        let b = Position::new(-7, 3);
        let sum = Position::new(-5, 2);
        assert_eq!(a + b, sum);
        assert_eq!(a + [-7, 3], sum);
    }

    #[test]
    fn test_sub() {
        let a = Position::new(2, -1);
        let b = Position::new(-7, 3);
        let diff = Position::new(9, -4);
        assert_eq!(a - b, diff);
        assert_eq!(a - [-7, 3], diff);
    }

    #[test]
    fn test_mul() {
        let a = Position::new(2, -1);
        let scale = 4;
        assert_eq!(a * scale, Position::new(8, -4));
    }

    #[test]
    fn test_div() {
        let a = Position::new(12, -10);
        let div = 4;
        assert_eq!(a / div, Position::new(3, -2));
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_zero() {
        let a = Position::new(12, -10);
        let div = 0;
        let _ = a / div;
    }
}
