use assert_approx_eq::assert_approx_eq;
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

mod clone {
    use super::*;

    #[test]
    fn it_clones_the_point() {
        let subject = Subject::new(1, 2).clone();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
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
        let point = &Point2i::new(1, 2);
        let subject: Point2f = point.into();

        assert_eq!(subject.x(), 1.0);
        assert_eq!(subject.y(), 2.0);
    }
}

mod addition {
    use super::*;

    #[test]
    fn it_can_add_one_point_to_another_to_return_a_point() {
        let a = Subject::new(1, 2);
        let b = Subject::new(4, 5);

        let subject: Subject<_> = &a + &b;

        assert_eq!(subject.x(), 5);
        assert_eq!(subject.y(), 7);
    }

    #[test]
    fn it_can_add_a_vector_to_the_point() {
        let point = Subject::new(1, 2);
        let vector = Vector2::new(3, 4);

        let subject: Subject<_> = &point + &vector;

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

        let vector: Vector2<_> = &a - &b;

        assert_eq!(vector.x(), 4);
        assert_eq!(vector.y(), 3);
    }

    #[test]
    fn it_can_subtract_a_vector_from_the_point() {
        let point = Subject::new(5, 5);
        let vector = Vector2::new(1, 2);

        let subject: Subject<_> = &point - &vector;

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

mod distance {
    use super::*;

    #[test]
    fn it_returns_the_distance_between_two_points() {
        let a = Subject::new(1, 2);
        let b = Subject::new(5, 5);

        assert_eq!(a.distance(&b), 5.0);
    }
}

mod distance_squared {
    use super::*;

    #[test]
    fn it_returns_the_square_of_distance_between_two_points() {
        let a = Subject::new(1, 2);
        let b = Subject::new(5, 5);

        assert_eq!(a.distance_squared(&b), 25.0);
    }
}

mod multiplication {
    use super::*;

    #[test]
    fn it_multiplies_by_a_scalar() {
        let subject = &Subject::new(1, 2) * 3;

        assert_eq!(subject.x(), 3);
        assert_eq!(subject.y(), 6);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let mut subject = Subject::new(1, 2);

        subject *= 3;

        assert_eq!(subject.x(), 3);
        assert_eq!(subject.y(), 6);
    }
}

mod division {
    use super::*;

    #[test]
    fn it_divides_by_a_divisor() {
        let subject: Subject<_> = &Subject::new(1, 2) / 10;

        assert_approx_eq!(subject.x(), 0.1);
        assert_approx_eq!(subject.y(), 0.2);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let mut subject = Subject::new(1.0, 2.0);

        subject /= 10;

        assert_approx_eq!(subject.x(), 0.1);
        assert_approx_eq!(subject.y(), 0.2);
    }
}

mod lerp {
    use super::*;

    #[test]
    fn it_linearly_interpolates_between_two_points() {
        let a = Subject::new(1, 2);
        let b = Subject::new(5, 6);

        let subject = a.lerp(&b, 0.5);

        assert_eq!(subject.x(), 3.0);
        assert_eq!(subject.y(), 4.0);
    }
}

mod min {
    use super::*;

    #[test]
    fn it_returns_a_point_of_the_component_wise_minimums() {
        let a = Subject::new(1, 9);
        let b = Subject::new(9, 2);

        let subject = a.min(&b);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }
}

mod max {
    use super::*;

    #[test]
    fn it_returns_a_point_of_the_component_wise_maximums() {
        let a = Subject::new(1, 0);
        let b = Subject::new(0, 2);

        let subject = a.max(&b);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }
}

mod abs {
    use super::*;

    #[test]
    fn it_can_take_the_absolute_value_of_point2f_components() {
        let subject = Subject::new(-1.0, -2.0).abs();

        assert_eq!(subject.x(), 1.0);
        assert_eq!(subject.y(), 2.0);
    }

    #[test]
    fn it_can_take_the_absolute_value_of_point2i_components() {
        let subject = Subject::new(-1, -2).abs();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }
}

mod floor {
    use super::*;

    #[test]
    fn it_can_take_the_floor_of_point2f_components() {
        let subject = Subject::new(1.2, 2.3).floor();

        assert_eq!(subject.x(), 1.0);
        assert_eq!(subject.y(), 2.0);
    }
}

mod ceil {
    use super::*;

    #[test]
    fn it_can_take_the_ceiling_of_point2f_components() {
        let subject = Subject::new(1.2, 2.3).ceil();

        assert_eq!(subject.x(), 2.0);
        assert_eq!(subject.y(), 3.0);
    }
}

mod permute {
    use super::*;

    #[test]
    fn it_permutes_the_component_values_according_to_the_indexes() {
        let subject = Subject::new(5, 6);

        let permute_01 = subject.permute(0, 1);
        let permute_10 = subject.permute(1, 0);

        assert_eq!(permute_01.x(), 5);
        assert_eq!(permute_01.y(), 6);

        assert_eq!(permute_10.x(), 6);
        assert_eq!(permute_10.y(), 5);
    }
}
