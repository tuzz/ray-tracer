use super::*;
use crate::geometry::vector3::Vector3;

type Subject<T> = Point3<T>;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_three_dimensional_point() {
        let subject = Subject::new(1, 2, 3);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }
}

mod aliases {
    use super::*;

    #[test]
    fn it_has_a_type_alias_for_a_point_of_signed_integers() {
        Point3i::new(-1 as i32, 2, -3);
    }

    #[test]
    fn it_has_a_type_alias_for_a_point_of_double_precision_floats() {
        Point3f::new(0.1 as f64, 0.2, 0.3);
    }
}

mod default {
    use super::*;

    #[test]
    fn it_sets_x_y_and_z_to_zero() {
        let subject = Subject::<u32>::default();

        assert_eq!(subject.x(), 0);
        assert_eq!(subject.y(), 0);
        assert_eq!(subject.z(), 0);
    }
}

mod conversions {
    use super::*;

    #[test]
    fn it_can_build_a_point3_from_an_iterator() {
        let subject: Subject<_> = (1..4).into();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }

    #[test]
    fn it_can_build_a_point3_from_a_point3_with_different_component_types() {
        let subject: Point3f = Point3i::new(1, 2, 3).into();

        assert_eq!(subject.x(), 1.0);
        assert_eq!(subject.y(), 2.0);
        assert_eq!(subject.z(), 3.0);
    }
}

mod addition {
    use super::*;

    #[test]
    fn it_can_add_a_vector_to_the_point() {
        let point = Subject::new(1, 2, 3);
        let vector = Vector3::new(4, 5, 6);

        let subject: Subject<_> = point + vector;

        assert_eq!(subject.x(), 5);
        assert_eq!(subject.y(), 7);
        assert_eq!(subject.z(), 9);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let point = Subject::new(1, 2, 3);
        let vector = Vector3::new(4, 5, 6);

        let mut subject = point;
        subject += vector;

        assert_eq!(subject.x(), 5);
        assert_eq!(subject.y(), 7);
        assert_eq!(subject.z(), 9);
    }
}
