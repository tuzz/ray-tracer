use generic_array::ArrayLength;
use super::point::Point;

pub struct Bounds<T, N: ArrayLength<T>> {
    pub p_min: Point<T, N>,
    pub p_max: Point<T, N>,
}

impl<T: Clone, N: ArrayLength<T>> From<&Point<T, N>> for Bounds<T, N> {
    fn from(point: &Point<T, N>) -> Self {
        Self { p_min: point.clone(), p_max: point.clone() }
    }
}
