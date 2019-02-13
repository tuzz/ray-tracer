use super::*;
use crate::geometry::vector2::Vector2;

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

mod default {
    use super::*;

    #[test]
    fn it_sets_x_and_y_to_zero() {
        let subject = Subject::<u32>::default();

        assert_eq!(subject.x(), 0);
        assert_eq!(subject.y(), 0);
    }
}

mod conversions {
    use super::*;

    #[test]
    fn it_can_build_a_point2_from_an_iterator() {
        let subject: Subject<_> = (1..3).into();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }

    #[test]
    fn it_can_build_a_point2_from_a_point3_by_dropping_the_z_component() {
        let subject: Subject<_> = Point3::new(1, 2, 3).into();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }

    #[test]
    fn it_can_build_a_point2_from_a_point2_with_different_component_types() {
        let subject: Point2f = Point2i::new(1, 2).into();

        assert_eq!(subject.x(), 1.0);
        assert_eq!(subject.y(), 2.0);
    }
}

mod addition {
    use super::*;

    #[test]
    fn it_can_add_a_vector_to_the_point() {
        let point = Subject::new(1, 2);
        let vector = Vector2::new(3, 4);

        let subject: Subject<_> = point + &vector;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 6);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let point = Subject::new(1, 2);
        let vector = Vector2::new(3, 4);

        let mut subject = point;
        subject += &vector;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 6);
    }
}

mod subtraction {
    use super::*;

    #[test]
    fn it_can_subtract_one_point_from_another_to_return_a_vector() {
        let a = Subject::new(5, 5);
        let b = Subject::new(1, 2);

        let vector: Vector2<_> = a - &b;

        assert_eq!(vector.x(), 4);
        assert_eq!(vector.y(), 3);
    }

    #[test]
    fn it_can_subtract_a_vector_from_the_point() {
        let point = Subject::new(5, 5);
        let vector = Vector2::new(1, 2);

        let subject: Subject<_> = point - &vector;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 3);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let point = Subject::new(5, 5);
        let vector = Vector2::new(1, 2);

        let mut subject = point;
        subject -= &vector;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 3);
    }
}
