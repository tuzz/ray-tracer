use std::f64::INFINITY;
use super::point3::Point3f;
use super::vector3::Vector3f;
use crate::dummy::Medium;

struct Ray {
    o: Point3f,
    d: Vector3f,
    t_max: f64,
    time: f64,
    medium: Option<Medium>,
}

impl Ray {
    fn new(o: Point3f, d: Vector3f, t_max: Option<f64>, time: Option<f64>, medium: Option<Medium>) -> Self {
        let t_max = t_max.unwrap_or(INFINITY);
        let time = time.unwrap_or(0.0);

        Self { o, d, t_max, time, medium }
    }
}

#[cfg(test)]
mod test;
