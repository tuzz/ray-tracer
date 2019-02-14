use crate::dummy::Medium;
use super::point3::Point3f;
use super::vector3::Vector3f;
use super::ray::Ray;

#[derive(Default)]
pub struct RayDifferential {
    pub ray: Ray,

    pub has_differentials: bool,
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vector3f,
    pub ry_direction: Vector3f,
}

impl From<Ray> for RayDifferential {
    fn from(ray: Ray) -> Self {
        Self { ray, ..Self::default() }
    }
}

impl RayDifferential {
    pub fn new(o: Point3f, d: Vector3f, t_max: Option<f64>, time: Option<f64>, medium: Option<Medium>) -> Self {
        Ray::new(o, d, t_max, time, medium).into()
    }
}

#[cfg(test)]
mod test;
