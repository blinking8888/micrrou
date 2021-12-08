use std::ops::{Add, AddAssign};

#[derive(Debug)]
pub struct Skipper<T>
where
    T: Copy + AddAssign + Add<Output = T> + PartialOrd,
{
    current: T,
    max: T,
    skip: T,
}

impl<T> Skipper<T>
where
    T: Copy + AddAssign + Add<Output = T> + PartialOrd,
{
    pub fn new(start: T, skip: T, max: T) -> Self {
        Skipper {
            current: start,
            skip,
            max,
        }
    }
}

impl<T> Iterator for Skipper<T>
where
    T: Copy + AddAssign + Add<Output = T> + PartialOrd + From<i8>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.skip < 0.into() {
            if (self.current + self.skip) >= self.max {
                Some(self.current)
            } else {
                None
            }
        } else if (self.current + self.skip) <= self.max {
            Some(self.current)
        } else {
            None
        };

        self.current += self.skip;

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn skip_count() {
        let mut skipper = Skipper::new(0.0, 5.5, 11.0);
        assert_eq!(skipper.next(), Some(0.0));
        assert_eq!(skipper.next(), Some(5.5));
        assert_eq!(skipper.next(), None);
    }

    #[test]
    fn skip_count_neg() {
        let mut skipper = Skipper::new(-2, 3, 11);
        assert_eq!(skipper.next(), Some(-2));
        assert_eq!(skipper.next(), Some(1));
        assert_eq!(skipper.next(), Some(4));
        assert_eq!(skipper.next(), Some(7));
        assert_eq!(skipper.next(), None);
    }

    #[test]
    fn skip_count_backwards() {
        let mut skipper = Skipper::new(10, -1, -2);
        assert_eq!(skipper.next(), Some(10));
        assert_eq!(skipper.next(), Some(9));
        assert_eq!(skipper.next(), Some(8));
        assert_eq!(skipper.next(), Some(7));
        assert_eq!(skipper.next(), Some(6));
        assert_eq!(skipper.next(), Some(5));
        assert_eq!(skipper.next(), Some(4));
        assert_eq!(skipper.next(), Some(3));
        assert_eq!(skipper.next(), Some(2));
        assert_eq!(skipper.next(), Some(1));
        assert_eq!(skipper.next(), Some(0));
        assert_eq!(skipper.next(), Some(-1));
        assert_eq!(skipper.next(), None);
    }

    #[test]
    fn skip_with_gaps() {
        let gap = 1;
        let width = 2;
        let end = 9;
        let mut skipper = Skipper::new(-5, gap + width, end + gap);
        assert_eq!(skipper.next(), Some(-5));
        assert_eq!(skipper.next(), Some(-2));
        assert_eq!(skipper.next(), Some(1));
        assert_eq!(skipper.next(), Some(4));
        assert_eq!(skipper.next(), Some(7));
        assert_eq!(skipper.next(), None);
    }
}
