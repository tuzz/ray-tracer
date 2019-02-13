use assert_approx_eq::assert_approx_eq;
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
    fn it_can_add_one_point_to_another_to_return_a_point() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(4, 5, 6);

        let subject: Subject<_> = a + &b;

        assert_eq!(subject.x(), 5);
        assert_eq!(subject.y(), 7);
        assert_eq!(subject.z(), 9);
    }

    #[test]
    fn it_can_add_a_vector_to_the_point() {
        let point = Subject::new(1, 2, 3);
        let vector = Vector3::new(4, 5, 6);

        let subject: Subject<_> = point + &vector;

        assert_eq!(subject.x(), 5);
        assert_eq!(subject.y(), 7);
        assert_eq!(subject.z(), 9);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let point = Subject::new(1, 2, 3);
        let vector = Vector3::new(4, 5, 6);

        let mut subject = point;
        subject += &vector;

        assert_eq!(subject.x(), 5);
        assert_eq!(subject.y(), 7);
        assert_eq!(subject.z(), 9);
    }
}

mod subtraction {
    use super::*;

    #[test]
    fn it_can_subtract_one_point_from_another_to_return_a_vector() {
        let a = Subject::new(5, 5, 5);
        let b = Subject::new(1, 2, 3);

        let vector: Vector3<_> = a - &b;

        assert_eq!(vector.x(), 4);
        assert_eq!(vector.y(), 3);
        assert_eq!(vector.z(), 2);
    }

    #[test]
    fn it_can_subtract_a_vector_from_the_point() {
        let point = Subject::new(5, 5, 5);
        let vector = Vector3::new(1, 2, 3);

        let subject: Subject<_> = point - &vector;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 3);
        assert_eq!(subject.z(), 2);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let point = Subject::new(5, 5, 5);
        let vector = Vector3::new(1, 2, 3);

        let mut subject = point;
        subject -= &vector;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 3);
        assert_eq!(subject.z(), 2);
    }
}

mod distance {
    use super::*;

    #[test]
    fn it_returns_the_distance_between_two_points() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(5, 5, 5);

        assert_eq!(a.distance(&b), f64::sqrt(29.0));
    }
}

mod distance_squared {
    use super::*;

    #[test]
    fn it_returns_the_square_of_distance_between_two_points() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(5, 5, 5);

        assert_eq!(a.distance_squared(&b), 29.0);
    }
}

mod multiplication {
    use super::*;

    #[test]
    fn it_multiplies_by_a_scalar() {
        let subject = Subject::new(1, 2, 3) * 4;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 8);
        assert_eq!(subject.z(), 12);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let mut subject = Subject::new(1, 2, 3);

        subject *= 4;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 8);
        assert_eq!(subject.z(), 12);
    }
}

mod division {
    use super::*;

    #[test]
    fn it_divides_by_a_divisor() {
        let subject: Subject<_> = Subject::new(1, 2, 3) / 10;

        assert_approx_eq!(subject.x(), 0.1);
        assert_approx_eq!(subject.y(), 0.2);
        assert_approx_eq!(subject.z(), 0.3);
    }

    #[test]
    fn it_can_mutate_the_point() {
        let mut subject = Subject::new(1.0, 2.0, 3.0);

        subject /= 10;

        assert_approx_eq!(subject.x(), 0.1);
        assert_approx_eq!(subject.y(), 0.2);
        assert_approx_eq!(subject.z(), 0.3);
    }
}

mod lerp {
    use super::*;

    #[test]
    fn it_linearly_interpolates_between_two_points() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(5, 6, 7);

        let subject = a.lerp(&b, 0.5);

        assert_eq!(subject.x(), 3.0);
        assert_eq!(subject.y(), 4.0);
        assert_eq!(subject.z(), 5.0);
    }
}

mod min {
    use super::*;

    #[test]
    fn it_returns_a_point_of_the_component_wise_minimums() {
        let a = Subject::new(1, 9, 3);
        let b = Subject::new(9, 2, 9);

        let subject = a.min(&b);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }
}

mod max {
    use super::*;

    #[test]
    fn it_returns_a_point_of_the_component_wise_maximums() {
        let a = Subject::new(1, 0, 3);
        let b = Subject::new(0, 2, 0);

        let subject = a.max(&b);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }
}

mod abs {
    use super::*;

    #[test]
    fn it_can_take_the_absolute_value_of_point2f_components() {
        let subject = Subject::new(-1.0, -2.0, -3.0).abs();

        assert_eq!(subject.x(), 1.0);
        assert_eq!(subject.y(), 2.0);
        assert_eq!(subject.z(), 3.0);
    }

    #[test]
    fn it_can_take_the_absolute_value_of_point2i_components() {
        let subject = Subject::new(-1, -2, -3).abs();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }
}
