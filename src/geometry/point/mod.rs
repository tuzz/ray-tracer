use generic_array::{ArrayLength, GenericArray};

#[derive(Default)]
pub struct Point<T, N: ArrayLength<T>> {
    pub components: GenericArray<T, N>,
}
