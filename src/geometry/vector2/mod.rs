use generic_array::{arr, arr_impl, typenum::U2};
use super::vector::*;

pub type Vector2<T> = Vector<T, U2>;

impl<T: Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { components: arr![T; x, y] }
    }

    pub fn x(&self) -> T {
        self.components[0]
    }

    pub fn y(&self) -> T {
        self.components[1]
    }

    pub fn permute(&self, x: usize, y: usize) -> Self {
        Self::new(self.components[x], self.components[y])
    }
}

pub type Vector2f = Vector2<f64>;
pub type Vector2i = Vector2<i32>;

#[cfg(test)]
mod test;
