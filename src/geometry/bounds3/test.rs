use super::*;

type Subject<T> = Bounds3<T>;

mod default {
    use super::*;

    #[test]
    fn it_builds_an_empty_bounding_box_for_f64() {
        let subject = Subject::<f64>::default();

        assert_eq!(subject.p_min.x(), std::f64::MAX);
        assert_eq!(subject.p_max.y(), std::f64::MIN);
    }

    #[test]
    fn it_builds_an_empty_bounding_box_for_i32() {
        let subject = Subject::<i32>::default();

        assert_eq!(subject.p_min.y(), std::i32::MAX);
        assert_eq!(subject.p_max.z(), std::i32::MIN);
    }
}

mod aliases {
    use super::*;

    #[test]
    fn it_has_a_type_alias_for_bounds_of_signed_integers() {
        Bounds3i::default();
    }

    #[test]
    fn it_has_a_type_alias_for_a_point_of_double_precision_floats() {
        Bounds3f::default();
    }
}

mod conversions {
    use super::*;

    #[test]
    fn it_can_build_a_bounding_box_from_a_point() {
        let point = &Point3::new(1.0, 2.0, 3.0);
        let subject: Subject<_> = point.into();

        assert_eq!(&subject.p_min, point);
        assert_eq!(&subject.p_max, point);
    }
}
