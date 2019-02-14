use generic_array::{arr, arr_impl, typenum::U3};
use super::normal::*;

pub type Normal3<T> = Normal<T, U3>;

impl<T: Copy> Normal3<T> {
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

pub type Normal3f = Normal3<f64>;

#[cfg(test)]
mod test;
