use super::vector3::Vector3f;

struct CoordinateSystem {
    v1: Vector3f,
    v2: Vector3f,
    v3: Vector3f,
}

impl CoordinateSystem {
    fn new(v1: &Vector3f) -> Self {
        let v1 = v1.clone();

        let v2 = match v1.x().abs() > v1.y().abs() {
            true => Vector3f::new(-v1.z(), 0.0, v1.x()),
            false => Vector3f::new(0.0, v1.z(), -v1.y()),
        }.normalize();

        let v3 = v1.cross(&v2);

        Self { v1, v2, v3 }
    }
}

#[cfg(test)]
mod test;
