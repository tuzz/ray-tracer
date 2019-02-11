use std::ops::{Index, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::cmp::{min, max};

#[derive(Clone, Default)]
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

impl<T: Add<Output=T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;

        Self::new(x, y, z)
    }
}

impl<T: AddAssign> AddAssign for Vector3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Sub<Output=T>> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        Self::new(x, y, z)
    }
}

impl<T: SubAssign> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        let x = self.x * scalar;
        let y = self.y * scalar;
        let z = self.z * scalar;

        Self::new(x, y, z)
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl<T: Into<f64>, D: Into<f64>> Div<D> for Vector3<T> {
    type Output = Vector3f;

    fn div(self, divisor: D) -> Vector3f {
        let inverse = 1.0 / divisor.into();

        let x = self.x.into() * inverse;
        let y = self.y.into() * inverse;
        let z = self.z.into() * inverse;

        Vector3f::new(x, y, z)
    }
}

impl<D: Into<f64>> DivAssign<D> for Vector3f {
    fn div_assign(&mut self, divisor: D) {
        let inverse = divisor.into().recip();

        self.x *= inverse;
        self.y *= inverse;
        self.z *= inverse;
    }
}

impl<T: Neg<Output=T>> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Vector3f {
    fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    fn abs_dot(&self, other: &Self) -> Self {
        self.dot(other).abs()
    }
}

impl Vector3i {
    fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    fn abs_dot(&self, other: &Self) -> Self {
        self.dot(other).abs()
    }
}

impl<T: Mul<Output=T> + Copy> Vector3<T> {
    fn dot(&self, other: &Self) -> Self {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        Self::new(x, y, z)
    }
}

impl<T: Into<f64> + Copy> Vector3<T> {
    fn cross(&self, other: &Self) -> Vector3f {
        let self_x = self.x.into();
        let self_y = self.y.into();
        let self_z = self.z.into();

        let other_x = other.x.into();
        let other_y = other.y.into();
        let other_z = other.z.into();

        let x = (self_y * other_z) - (self_z * other_y);
        let y = (self_z * other_x) - (self_x * other_z);
        let z = (self_x * other_y) - (self_y * other_x);

        Vector3f::new(x, y, z)
    }
}

impl<T: Into<f64> + Copy> Vector3<T> {
    fn length_squared(&self) -> f64 {
        let x = self.x.into();
        let y = self.y.into();
        let z = self.z.into();

        x * x + y * y + z * z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn normalize(&self) -> Vector3f {
        self.clone() / self.length()
    }
}

impl<T: Ord + Copy> Vector3<T> {
    fn min_component(&self) -> T {
        min(self.x, min(self.y, self.z))
    }

    fn max_component(&self) -> T {
        max(self.x, max(self.y, self.z))
    }

    fn min_dimension(&self) -> usize {
        if self.x < self.y {
            if self.x < self.z { 0 } else { 2 }
        } else {
            if self.y < self.z { 1 } else { 2 }
        }
    }

    fn max_dimension(&self) -> usize {
        if self.x > self.y {
            if self.x > self.z { 0 } else { 2 }
        } else {
            if self.y > self.z { 1 } else { 2 }
        }
    }
}

#[cfg(test)]
mod test;
