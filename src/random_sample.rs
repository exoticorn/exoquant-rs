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

pub trait RandomSample<T: Iterator> {
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
