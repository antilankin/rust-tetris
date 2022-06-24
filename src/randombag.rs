use crate::tetromino::Shape;

struct RandomBag {
    contents: [Shape; 14],
}

impl RandomBag {
    fn default() -> RandomBag {
        RandomBag {
            contents: [
                Shape::I,
                Shape::J,
                Shape::L,
                Shape::O,
                Shape::S,
                Shape::T,
                Shape::Z,
                Shape::I,
                Shape::J,
                Shape::L,
                Shape::O,
                Shape::S,
                Shape::T,
                Shape::Z,
            ],
        }
    }

    fn peek(self) -> Shape {
        self.contents[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag() {
        let bag = RandomBag::default();
        assert_eq!(bag.peek(), Shape::I);
    }
}
