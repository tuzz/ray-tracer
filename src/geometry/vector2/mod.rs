use std::ops::Index;

struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds: {}", index),
        }
    }
}

#[cfg(test)]
mod test;
