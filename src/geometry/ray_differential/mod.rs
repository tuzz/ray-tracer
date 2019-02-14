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

    pub fn scale_differentials(&mut self, s: f64) {
        let o = &self.ray.o;
        let d = &self.ray.d;

        self.rx_origin = o + &(&(&self.rx_origin - o) * s);
        self.ry_origin = o + &(&(&self.ry_origin - o) * s);

        self.rx_direction = d + &(&(&self.rx_direction - d) * s);
        self.ry_direction = d + &(&(&self.ry_direction - d) * s);
    }
}

#[cfg(test)]
mod test;
