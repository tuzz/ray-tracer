use generic_array::{ArrayLength, GenericArray};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::vector::Vector;

#[derive(Default)]
pub struct Point<T, N: ArrayLength<T>> {
    pub components: GenericArray<T, N>,
}

impl<T, N: ArrayLength<T>, I, X> From<I> for Point<T, N>
    where I: IntoIterator<Item=T, IntoIter=X>,
          X: ExactSizeIterator<Item=T>,
{
    fn from(iter: I) -> Self {
        Self { components: GenericArray::from_exact_iter(iter).unwrap() }
    }
}

// NotEq is a marker trait that's auto-implemented on all 2-tuples...
pub auto trait NotEq { }

// ...except those with two values of the same type...
impl<T> !NotEq for (T, T) { }

impl<T, U, N: ArrayLength<T> + ArrayLength<U>> From<Point<U, N>> for Point<T, N>
    where (T, U): NotEq, // ...which lets us add a trait bound that avoids a conflict
          T: From<U>,    //    with the auto-implemented From<T> trait.
          U: Copy,
{
    fn from(p: Point<U, N>) -> Self {
        p.components.iter().map(|&a| a.into()).into()
    }
}

impl<T: Add<Output=T> + Copy, N: ArrayLength<T>> Add<Vector<T, N>> for Point<T, N> {
    type Output = Self;

    fn add(self, vector: Vector<T, N>) -> Self::Output {
        self.components.iter().zip(vector.components).map(|(a, b)| *a + b).into()
    }
}

impl<T: AddAssign + Copy, N: ArrayLength<T>> AddAssign<Vector<T, N>> for Point<T, N> {
    fn add_assign(&mut self, vector: Vector<T, N>) {
        self.components.iter_mut().zip(vector.components).for_each(|(a, b)| *a += b);
    }
}

impl<T: Sub<Output=T> + Copy, N: ArrayLength<T>> Sub<Vector<T, N>> for Point<T, N> {
    type Output = Self;

    fn sub(self, vector: Vector<T, N>) -> Self::Output {
        self.components.iter().zip(vector.components).map(|(a, b)| *a - b).into()
    }
}

impl<T: SubAssign + Copy, N: ArrayLength<T>> SubAssign<Vector<T, N>> for Point<T, N> {
    fn sub_assign(&mut self, vector: Vector<T, N>) {
        self.components.iter_mut().zip(vector.components).for_each(|(a, b)| *a -= b);
    }
}
