use generic_array::{ArrayLength, GenericArray};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::iter::Sum;
use super::vector::Vector;

#[derive(Default)]
pub struct Normal<T, N: ArrayLength<T>> {
    pub components: GenericArray<T, N>,
}

impl<T, N: ArrayLength<T>, I, X> From<I> for Normal<T, N>
    where I: IntoIterator<Item=T, IntoIter=X>,
          X: ExactSizeIterator<Item=T>,
{
    fn from(iter: I) -> Self {
        Self { components: GenericArray::from_exact_iter(iter).unwrap() }
    }
}

impl <T, N: ArrayLength<T>> From<Vector<T, N>> for Normal<T, N> {
    fn from(vector: Vector<T, N>) -> Self {
        Self { components: vector.components }
    }
}

impl<T: Add<Output=T> + Copy, N: ArrayLength<T>> Add<&Self> for Normal<T, N> {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        self.components.iter()
            .zip(other.components.iter())
            .map(|(a, b)| *a + *b)
            .into()
    }
}

impl<T: AddAssign + Copy, N: ArrayLength<T>> AddAssign<&Self> for Normal<T, N> {
    fn add_assign(&mut self, other: &Self) {
        self.components.iter_mut()
            .zip(other.components.iter())
            .for_each(|(a, b)| *a += *b);
    }
}

impl<T: Sub<Output=T> + Copy, N: ArrayLength<T>> Sub<&Self> for Normal<T, N> {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        self.components.iter()
            .zip(other.components.iter())
            .map(|(a, b)| *a - *b)
            .into()
    }
}

impl<T: SubAssign + Copy, N: ArrayLength<T>> SubAssign<&Self> for Normal<T, N> {
    fn sub_assign(&mut self, other: &Self) {
        self.components.iter_mut()
            .zip(other.components.iter())
            .for_each(|(a, b)| *a -= *b);
    }
}

impl<T: Mul<Output=T> + Copy, N: ArrayLength<T>> Mul<T> for Normal<T, N> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        self.components.iter().map(|a| *a * scalar).into()
    }
}

impl<T: MulAssign + Copy, N: ArrayLength<T>> MulAssign<T> for Normal<T, N> {
    fn mul_assign(&mut self, scalar: T) {
        self.components.iter_mut().for_each(|a| *a *= scalar);
    }
}

impl<T: Into<f64> + Copy, D: Into<f64>, N: ArrayLength<T> + ArrayLength<f64>> Div<D> for Normal<T, N> {
    type Output = Normal<f64, N>;

    fn div(self, divisor: D) -> Self::Output {
        let inverse = divisor.into().recip();

        self.components.iter().map(|&a| a.into() * inverse).into()
    }
}

impl<D: Into<f64>, N: ArrayLength<f64>> DivAssign<D> for Normal<f64, N> {
    fn div_assign(&mut self, divisor: D) {
        let inverse = divisor.into().recip();

        self.components.iter_mut().for_each(|a| *a *= inverse);
    }
}

impl<T: Neg<Output=T> + Copy, N: ArrayLength<T>> Neg for Normal<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.components.iter().map(|&a| -a).into()
    }
}

impl<N: ArrayLength<f64>> Normal<f64, N> {
    pub fn abs(&self) -> Self {
        self.components.iter().map(|&a| a.abs()).into()
    }

    pub fn abs_dot<S: Into<Self> + Clone>(&self, other: &S) -> f64 {
        self.dot(other).abs()
    }
}

impl<T: Mul<Output=T> + Sum + Copy, N: ArrayLength<T>> Normal<T, N> {
    pub fn dot<S: Into<Self> + Clone>(&self, other: &S) -> T {
        let other = other.clone().into();

        self.components.iter()
            .zip(other.components.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }
}

impl<T: Into<f64> + Copy, N: ArrayLength<T>> Normal<T, N> {
    pub fn length_squared(&self) -> f64 {
        self.components.iter().map(|&a| a.into().powf(2.0)).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl<T: Into<f64> + Copy, N: ArrayLength<T> + ArrayLength<f64>> Normal<T, N> {
    pub fn normalize(&self) -> Normal<f64, N> {
        self.clone() / self.length()
    }
}

// Cloning GenericArray doesn't seem to work properly, so implement it manually:
impl<T: Clone, N: ArrayLength<T>> Clone for Normal<T, N> {
    fn clone(&self) -> Self {
        self.components.iter().map(|a| a.clone()).into()
    }
}

impl<T, N: ArrayLength<T>> Normal<T, N>
    where T: Mul<Output=T> + Sum + Copy, // To satisfy dot.
          T: Ord + Default,              // For the comparison.
          T: Neg<Output=T>               // To satisfy neg.
{
    pub fn face_forward<S: Into<Self> + Clone>(&self, other: &S) -> Self {
        if self.dot(other.into()) < T::default() {
            -self.clone()
        } else {
            self.clone()
        }
    }
}
