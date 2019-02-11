use std::ops::Index;

struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

type Vector3f = Vector3<f64>;
type Vector3i = Vector3<i32>;

impl<T> Vector3<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: {}", index),
        }
    }
}

#[cfg(test)]
mod test;
