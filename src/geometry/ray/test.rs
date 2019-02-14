use super::*;
use crate::dummy::Medium;

type Subject = Ray;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_ray_and_sets_its_fields() {
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

        assert_eq!(subject.o, origin);
        assert_eq!(subject.d, direction);
        assert_eq!(subject.t_max, 123.0);
        assert_eq!(subject.time, 456.0);
        assert_eq!(subject.medium, medium);
    }
}
