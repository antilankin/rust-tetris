use crate::tetromino::{all_shapes, Shape};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub struct RandomBag {
    contents: [Shape; 14],
    index: usize,
    rng: ThreadRng,
}

impl RandomBag {
    pub fn new() -> Self {
        let mut bag = RandomBag {
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
            rng: thread_rng(),
        };
        bag.shuffle_front();
        bag.shuffle_back();
        bag
    }

    fn peek(&self) -> Shape {
        self.contents[self.index]
    }

    pub fn get(&mut self) -> Shape {
        let result = self.contents[self.index];
        self.index = (self.index + 1) % self.contents.len();
        match self.index {
            0 => self.shuffle_back(),
            7 => self.shuffle_front(),
            _ => (),
        }
        result
    }

    fn shuffle_front(&mut self) {
        self.contents[0..7].shuffle(&mut self.rng);
    }

    fn shuffle_back(&mut self) {
        self.contents[7..14].shuffle(&mut self.rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_get() {
        let mut bag = RandomBag::new();
        for _ in 1..=140 {
            assert_eq!(bag.peek(), bag.get());
        }
    }

    #[test]
    fn test_count() {
        let bag = RandomBag::new();
        let count_shapes_in_slice = |begin: usize, end: usize, shape: &Shape| {
            bag.contents[begin..end]
                .iter()
                .filter(|bag_shape| *bag_shape == shape)
                .count()
        };
        for shape in all_shapes() {
            assert_eq!(count_shapes_in_slice(0, 7, &shape), 1);
            assert_eq!(count_shapes_in_slice(7, 14, &shape), 1);
        }
    }
}
