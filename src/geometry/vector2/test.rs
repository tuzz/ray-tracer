use assert_approx_eq::assert_approx_eq;
use super::*;
use crate::geometry::point2::Point2;

type Subject<T> = Vector2<T>;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_two_dimensional_vector() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }
}

mod components {
    use super::*;

    #[test]
    fn it_can_iterate_over_components_of_the_struct() {
        let subject = Subject::new(1, 2);
        let iter = subject.components.iter();

        assert_eq!(iter.as_slice(), &[1, 2]);
    }
}

mod aliases {
    use super::*;

    #[test]
    fn it_has_a_type_alias_for_a_vector_of_signed_integers() {
        Vector2i::new(-1 as i32, 2);
    }

    #[test]
    fn it_has_a_type_alias_for_a_vector_of_double_precision_floats() {
        Vector2f::new(0.1 as f64, 0.2);
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
    fn it_can_build_a_vector2_from_an_iterator() {
        let subject: Subject<_> = (1..3).into();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }

    #[test]
    fn it_can_build_a_vector2_from_a_point2() {
        let subject: Subject<_> = Point2::new(1, 2).into();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }
}

mod addition {
    use super::*;

    #[test]
    fn it_adds_the_components() {
        let a = Subject::new(1, 2);
        let b = Subject::new(3, 4);

        let subject = a + &b;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 6);
    }

    #[test]
    fn it_can_mutate_the_vector() {
        let a = Subject::new(1, 2);
        let b = Subject::new(3, 4);

        let mut subject = a;
        subject += &b;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 6);
    }
}

mod subtraction {
    use super::*;

    #[test]
    fn it_subtracts_the_components() {
        let a = Subject::new(5, 5);
        let b = Subject::new(1, 2);

        let subject = a - &b;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 3);
    }

    #[test]
    fn it_can_mutate_the_vector() {
        let a = Subject::new(5, 5);
        let b = Subject::new(1, 2);

        let mut subject = a;
        subject -= &b;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 3);
    }
}

mod multiplication {
    use super::*;

    #[test]
    fn it_multiplies_by_a_scalar() {
        let subject = Subject::new(1, 2) * 3;

        assert_eq!(subject.x(), 3);
        assert_eq!(subject.y(), 6);
    }

    #[test]
    fn it_can_mutate_the_vector() {
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
        let subject = Subject::new(1, 2) / 10;

        assert_approx_eq!(subject.x(), 0.1);
        assert_approx_eq!(subject.y(), 0.2);
    }

    #[test]
    fn it_can_mutate_the_vector() {
        let mut subject = Subject::new(1.0, 2.0);

        subject /= 10;

        assert_approx_eq!(subject.x(), 0.1);
        assert_approx_eq!(subject.y(), 0.2);
    }
}

mod negation {
    use super::*;

    #[test]
    fn it_returns_a_vector_pointing_in_the_opposite_direction() {
        let subject = -Subject::new(1, 2);

        assert_eq!(subject.x(), -1);
        assert_eq!(subject.y(), -2);
    }
}

mod abs {
    use super::*;

    #[test]
    fn it_can_take_the_absolute_value_of_vector2f_components() {
        let subject = Subject::new(-1.0, -2.0).abs();

        assert_eq!(subject.x(), 1.0);
        assert_eq!(subject.y(), 2.0);
    }

    #[test]
    fn it_can_take_the_absolute_value_of_vector2i_components() {
        let subject = Subject::new(-1, -2).abs();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
    }
}

mod dot {
    use super::*;

    #[test]
    fn it_returns_the_dot_product_of_two_vectors() {
        let a = Subject::new(1, 2);
        let b = Subject::new(3, 4);

        let subject = a.dot(&b);

        assert_eq!(subject.x(), 3);
        assert_eq!(subject.y(), 8);
    }
}

mod abs_dot {
    use super::*;

    #[test]
    fn it_returns_the_absolute_value_of_the_dot_product() {
        let a = Subject::new(1, 2);
        let b = Subject::new(-3, -4);

        let subject = a.abs_dot(&b);

        assert_eq!(subject.x(), 3);
        assert_eq!(subject.y(), 8);
    }
}

mod length_squared {
    use super::*;

    #[test]
    fn it_returns_the_square_of_the_length_of_the_vector() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.length_squared(), 5.0);
    }
}

mod length {
    use super::*;

    #[test]
    fn it_returns_the_length_of_the_vector() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.length(), f64::sqrt(5.0));
    }
}

mod normalize {
    use super::*;

    #[test]
    fn it_divides_each_component_by_the_vectors_length() {
        let subject = Subject::new(1, 2).normalize();
        let divisor = f64::sqrt(5.0);

        assert_eq!(subject.x(), 1.0 / divisor);
        assert_eq!(subject.y(), 2.0 / divisor);
    }
}

mod min_component {
    use super::*;

    #[test]
    fn it_returns_the_value_of_the_smallest_component() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.min_component(), 1);
    }
}

mod max_component {
    use super::*;

    #[test]
    fn it_returns_the_value_of_the_largest_component() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.max_component(), 2);
    }
}

mod min_dimension {
    use super::*;

    #[test]
    fn it_returns_the_index_of_the_dimension_with_smallest_component() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.min_dimension(), 0);
    }
}

mod max_dimension {
    use super::*;

    #[test]
    fn it_returns_the_index_of_the_dimension_with_largest_component() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.max_dimension(), 1);
    }
}

mod min {
    use super::*;

    #[test]
    fn it_returns_a_vector_of_the_component_wise_minimums() {
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
    fn it_returns_a_vector_of_the_component_wise_maximums() {
        let a = Subject::new(1, 0);
        let b = Subject::new(0, 2);

        let subject = a.max(&b);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
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
