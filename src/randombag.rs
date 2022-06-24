use crate::tetromino::{all_shapes, Shape};

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
    fn test_peek_get() {
        let mut bag = RandomBag::default();
        for _ in 1..=140 {
            assert_eq!(bag.peek(), bag.get());
        }
    }

    #[test]
    fn test_count() {
        let mut bag = RandomBag::default();
        for shape in all_shapes() {
            assert_eq!(
                bag.contents[0..7]
                    .iter()
                    .filter(|bag_shape| **bag_shape == shape)
                    .count(),
                1
            );
            assert_eq!(
                bag.contents[7..14]
                    .iter()
                    .filter(|bag_shape| **bag_shape == shape)
                    .count(),
                1
            );
        }
    }
}
