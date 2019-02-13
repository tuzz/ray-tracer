use generic_array::{ArrayLength, GenericArray};

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
