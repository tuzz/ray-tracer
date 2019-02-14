use super::*;
use std::cell::RefCell;

type Subject = RayDifferential;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_ray_differential_and_sets_its_fields() {
        let origin = Point3f::new(1.0, 2.0, 3.0);
        let direction = Vector3f::new(4.0, 5.0, 6.0);
        let t_max = Some(123.0);
        let time = Some(456.0);
        let medium = Some(Medium::new());

        let subject = Subject::new(
            origin.clone(),
            direction.clone(),
            t_max,
            time,
            medium.clone(),
        );

        let ray = &subject.ray;

        assert_eq!(ray.o, origin);
        assert_eq!(ray.d, direction);
        assert_eq!(ray.t_max, RefCell::new(123.0));
        assert_eq!(ray.time, 456.0);
        assert_eq!(ray.medium, medium);

        assert_eq!(subject.rx_origin, Point3f::default());
        assert_eq!(subject.ry_origin, Point3f::default());
        assert_eq!(subject.rx_direction, Vector3f::default());
        assert_eq!(subject.ry_direction, Vector3f::default());
    }
}

mod conversions {
    use super::*;

    #[test]
    fn it_can_build_a_ray_differential_from_a_ray() {
        let origin = Point3f::new(1.0, 2.0, 3.0);
        let direction = Vector3f::new(4.0, 5.0, 6.0);
        let t_max = Some(123.0);
        let time = Some(456.0);
        let medium = Some(Medium::new());

        let ray = Ray::new(origin, direction, t_max, time, medium);
        let subject: Subject = ray.clone().into();

        assert_eq!(subject.ray, ray);
    }
}
