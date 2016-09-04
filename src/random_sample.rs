//! Random sampling of iterators

extern crate rand;

pub struct RandomSampleIter<'a, T: 'a + Iterator> {
    iter: &'a mut T,
    prob: f32,
}

impl<'a, T: Iterator> Iterator for RandomSampleIter<'a, T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<T::Item> {
        while let Some(color) = self.iter.next() {
            if self.prob >= rand::random() {
                return Some(color);
            }
        }
        None
    }
}

/// Adds a `random_sample(probability)` method to iterators.
pub trait RandomSample<T: Iterator> {
    /// Returns a new iterator that randomly samples the original iterator.
    fn random_sample<'a>(&'a mut self, prob: f32) -> RandomSampleIter<'a, T>;
}

impl<T: Iterator> RandomSample<T> for T {
    fn random_sample<'a>(&'a mut self, prob: f32) -> RandomSampleIter<'a, Self> {
        RandomSampleIter {
            iter: self,
            prob: prob,
        }
    }
}
