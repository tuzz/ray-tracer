struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod test;
