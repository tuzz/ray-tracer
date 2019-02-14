use generic_array::ArrayLength;
use super::point::Point;

pub struct Bounds<T, N: ArrayLength<T>> {
    pub p_min: Point<T, N>,
    pub p_max: Point<T, N>,
}
