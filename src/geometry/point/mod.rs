use generic_array::{ArrayLength, GenericArray};

pub struct Point<T, N: ArrayLength<T>> {
    pub components: GenericArray<T, N>,
}
