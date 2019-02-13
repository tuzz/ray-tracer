use generic_array::{arr, arr_impl, typenum::U3};
use super::vector::*;

pub type Vector3<T> = Vector<T, U3>;

impl<T: Copy> Vector3<T> {
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

    pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
        let c = &self.components;

        Self::new(c[x], c[y], c[z])
    }
}

// This isn't applicable in two dimensions and is hard to generalise to > 3 dimensions.
impl<T: Into<f64> + Copy> Vector3<T> {
    pub fn cross(&self, other: &Self) -> Vector3f {
        let v1 = self.components.iter().map(|&a| a.into()).collect::<Vec<f64>>();
        let v2 = other.components.iter().map(|&b| b.into()).collect::<Vec<f64>>();

        let x = (v1[1] * v2[2]) - (v1[2] * v2[1]);
        let y = (v1[2] * v2[0]) - (v1[0] * v2[2]);
        let z = (v1[0] * v2[1]) - (v1[1] * v2[0]);

        Vector3f::new(x, y, z)
    }
}

pub type Vector3f = Vector3<f64>;
pub type Vector3i = Vector3<i32>;

#[cfg(test)]
mod test;
