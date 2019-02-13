use generic_array::{arr, arr_impl, typenum::U3};
use super::point::*;

pub type Point3<T> = Point<T, U3>;

impl<T: Copy> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { components: arr![T; x, y, z] }
    }

    pub fn x(&self) -> T {
        self.components[0]
    }

    pub fn y(&self) -> T {
        self.components[1]
    }

    pub fn z(&self) -> T {
        self.components[2]
    }
}

pub type Point3f = Point3<f64>;
pub type Point3i = Point3<i32>;

#[cfg(test)]
mod test;
