use crate::tetromino::Shape;

struct RandomBag {
    contents: [Shape; 14],
    index: usize,
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
            index: 0,
        }
    }

    fn peek(&self) -> Shape {
        self.contents[self.index]
    }

    fn get(&mut self) -> Shape {
        let result = self.contents[self.index];
        self.index = (self.index + 1) % self.contents.len();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut bag = RandomBag::default();
        for _ in 1..=3 {
            assert_eq!(bag.peek(), Shape::I);
            assert_eq!(bag.get(), Shape::I);
            assert_eq!(bag.peek(), Shape::J);
            assert_eq!(bag.get(), Shape::J);
            assert_eq!(bag.peek(), Shape::L);
            assert_eq!(bag.get(), Shape::L);
            assert_eq!(bag.peek(), Shape::O);
            assert_eq!(bag.get(), Shape::O);
            assert_eq!(bag.peek(), Shape::S);
            assert_eq!(bag.get(), Shape::S);
            assert_eq!(bag.peek(), Shape::T);
            assert_eq!(bag.get(), Shape::T);
            assert_eq!(bag.peek(), Shape::Z);
            assert_eq!(bag.get(), Shape::Z);
        }
    }
}
