use generic_array::{arr, arr_impl, typenum::U2, sequence::Shorten};
use super::point::*;
use super::point3::Point3;

pub type Point2<T> = Point<T, U2>;

impl<T: Copy> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { components: arr![T; x, y] }
    }

    pub fn x(&self) -> T {
        self.components[0]
    }

    pub fn y(&self) -> T {
        self.components[1]
    }
}

impl<T> From<Point3<T>> for Point2<T> {
    fn from(p: Point3<T>) -> Self {
        let (components, _) = p.components.pop_back();

        Self { components }
    }
}

pub type Point2f = Point2<f64>;
pub type Point2i = Point2<i32>;

#[cfg(test)]
mod test;
