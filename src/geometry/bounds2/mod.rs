use generic_array::typenum::U2;
use crate::geometry::point2::Point2;
use super::bounds::*;

pub type Bounds2<T> = Bounds<T, U2>;

impl Bounds2<f64> {
    pub fn new() -> Self {
        let min = std::f64::MIN;
        let max = std::f64::MAX;

        let p_min = Point2::new(max, max);
        let p_max = Point2::new(min, min);

        Self { p_min, p_max }
    }
}

impl Bounds2<i32> {
    pub fn new() -> Self {
        let min = std::i32::MIN;
        let max = std::i32::MAX;

        let p_min = Point2::new(max, max);
        let p_max = Point2::new(min, min);

        Self { p_min, p_max }
    }
}

#[cfg(test)]
mod test;
