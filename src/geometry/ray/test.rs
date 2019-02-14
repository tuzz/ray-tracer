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
        assert_eq!(subject.t_max, RefCell::new(123.0));
        assert_eq!(subject.time, 456.0);
        assert_eq!(subject.medium, medium);
    }
}

mod default {
    use super::*;

    #[test]
    fn it_sets_sensible_defaults_for_the_fields() {
        let subject = Subject::default();

        assert_eq!(subject.o, Point3f::default());
        assert_eq!(subject.d, Vector3f::default());
        assert_eq!(subject.t_max, RefCell::new(INFINITY));
        assert_eq!(subject.time, 0.0);
        assert_eq!(subject.medium, None);
    }
}

mod interior_mutability {
    use super::*;

    #[test]
    fn it_allows_t_max_to_be_mutated_when_the_ray_is_immutable() {
        let subject = Subject::default();

        *subject.t_max.borrow_mut() = 555.0;

        assert_eq!(subject.t_max, RefCell::new(555.0));
    }
}
