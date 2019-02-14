use std::f64::INFINITY;
use std::cell::RefCell;
use super::point3::Point3f;
use super::vector3::Vector3f;
use crate::dummy::Medium;

struct Ray {
    pub o: Point3f,
    pub d: Vector3f,
    pub t_max: RefCell<f64>,
    pub time: f64,
    pub medium: Option<Medium>,
}

impl Ray {
    fn new(o: Point3f, d: Vector3f, t_max: Option<f64>, time: Option<f64>, medium: Option<Medium>) -> Self {
        let t_max = RefCell::new(t_max.unwrap_or(INFINITY));
        let time = time.unwrap_or(0.0);

        Self { o, d, t_max, time, medium }
    }

    fn at(&self, time: f64) -> Point3f {
        let origin = &self.o;
        let offset = &self.d * time;

        origin + &offset
    }
}

impl Default for Ray {
    fn default() -> Self {
        let o = Point3f::default();
        let d = Vector3f::default();
        let t_max = Some(INFINITY);
        let time = Some(0.0);
        let medium = None;

        Ray::new(o, d, t_max, time, medium)
    }
}

#[cfg(test)]
mod test;
