struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod test;
