use generic_array::typenum::U3;
use crate::geometry::point3::Point3;
use super::bounds::*;

pub type Bounds3<T> = Bounds<T, U3>;

impl Bounds3<f64> {
    pub fn new() -> Self {
        let min = std::f64::MIN;
        let max = std::f64::MAX;

        let p_min = Point3::new(max, max, max);
        let p_max = Point3::new(min, min, min);

        Self { p_min, p_max }
    }
}

impl Bounds3<i32> {
    pub fn new() -> Self {
        let min = std::i32::MIN;
        let max = std::i32::MAX;

        let p_min = Point3::new(max, max, max);
        let p_max = Point3::new(min, min, min);

        Self { p_min, p_max }
    }
}

#[cfg(test)]
mod test;
