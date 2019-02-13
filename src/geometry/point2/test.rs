use super::*;

type Subject<T> = Point2<T>;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_two_dimensional_point() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }
}

mod aliases {
    use super::*;

    #[test]
    fn it_has_a_type_alias_for_a_point_of_signed_integers() {
        Point2i::new(-1 as i32, 2);
    }

    #[test]
    fn it_has_a_type_alias_for_a_point_of_double_precision_floats() {
        Point2f::new(0.1 as f64, 0.2);
    }
}
