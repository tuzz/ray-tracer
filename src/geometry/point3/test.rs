use super::*;

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
