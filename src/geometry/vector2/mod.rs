use std::ops::{Index, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::cmp::{min, max};

#[derive(Clone, Default)]
struct Vector2<T> {
    pub x: T,
    pub y: T,
}

type Vector2f = Vector2<f64>;
type Vector2i = Vector2<i32>;

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

impl<T: Add<Output=T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;

        Self::new(x, y)
    }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub<Output=T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;

        Self::new(x, y)
    }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        let x = self.x * scalar;
        let y = self.y * scalar;

        Self::new(x, y)
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl<T: Into<f64>, D: Into<f64>> Div<D> for Vector2<T> {
    type Output = Vector2f;

    fn div(self, divisor: D) -> Vector2f {
        let inverse = divisor.into().recip();

        let x = self.x.into() * inverse;
        let y = self.y.into() * inverse;

        Vector2f::new(x, y)
    }
}

impl<D: Into<f64>> DivAssign<D> for Vector2f {
    fn div_assign(&mut self, divisor: D) {
        let inverse = 1.0 / divisor.into();

        self.x *= inverse;
        self.y *= inverse;
    }
}

impl<T: Neg<Output=T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Vector2f {
    fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    fn abs_dot(&self, other: &Self) -> Self {
        self.dot(other).abs()
    }
}

impl Vector2i {
    fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    fn abs_dot(&self, other: &Self) -> Self {
        self.dot(other).abs()
    }
}

impl<T: Mul<Output=T> + Copy> Vector2<T> {
    fn dot(&self, other: &Self) -> Self {
        let x = self.x * other.x;
        let y = self.y * other.y;

        Self::new(x, y)
    }
}

impl<T: Into<f64> + Copy> Vector2<T> {
    fn length_squared(&self) -> f64 {
        let x = self.x.into();
        let y = self.y.into();

        x * x + y * y
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn normalize(&self) -> Vector2f {
        self.clone() / self.length()
    }
}

impl<T: Ord + Copy> Vector2<T> {
    fn min_component(&self) -> T {
        min(self.x, self.y)
    }

    fn max_component(&self) -> T {
        max(self.x, self.y)
    }

    fn min_dimension(&self) -> usize {
        if self.x < self.y { 0 } else { 1 }
    }

    fn max_dimension(&self) -> usize {
        if self.x > self.y { 0 } else { 1 }
    }

    fn min(&self, other: &Self) -> Self {
        let x = min(self.x, other.x);
        let y = min(self.y, other.y);

        Self::new(x, y)
    }

    fn max(&self, other: &Self) -> Self {
        let x = max(self.x, other.x);
        let y = max(self.y, other.y);

        Self::new(x, y)
    }
}

impl<T: Copy> Vector2<T> {
    fn permute(&self, x: usize, y: usize) -> Self {
        Self::new(self[x], self[y])
    }
}

#[cfg(test)]
mod test;
