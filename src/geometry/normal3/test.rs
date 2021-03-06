use super::*;
use assert_approx_eq::assert_approx_eq;
use crate::geometry::vector3::Vector3;

type Subject<T> = Normal3<T>;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_three_dimensional_normal() {
        let subject = Subject::new(1, 2, 3);

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }
}

mod components {
    use super::*;

    #[test]
    fn it_can_iterate_over_components_of_the_struct() {
        let subject = Subject::new(1, 2, 3);
        let iter = subject.components.iter();

        assert_eq!(iter.as_slice(), &[1, 2, 3]);
    }
}

mod aliases {
    use super::*;

    #[test]
    fn it_has_a_type_alias_for_a_normal_of_double_precision_floats() {
        Normal3f::new(0.1 as f64, 0.2, 0.3);
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

mod clone {
    use super::*;

    #[test]
    fn it_clones_the_normal() {
        let subject = Subject::new(1, 2, 3).clone();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }
}

mod conversions {
    use super::*;

    #[test]
    fn it_can_build_a_normal3_from_an_iterator() {
        let subject: Subject<_> = (1..4).into();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }

    #[test]
    fn it_can_build_a_normal3_from_a_vector3() {
        let vector = &Vector3::new(1, 2, 3);
        let subject: Subject<_> = vector.into();

        assert_eq!(subject.x(), 1);
        assert_eq!(subject.y(), 2);
        assert_eq!(subject.z(), 3);
    }
}

mod addition {
    use super::*;

    #[test]
    fn it_adds_the_components() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(4, 5, 6);

        let subject = &a + &b;

        assert_eq!(subject.x(), 5);
        assert_eq!(subject.y(), 7);
        assert_eq!(subject.z(), 9);
    }

    #[test]
    fn it_can_mutate_the_normal() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(4, 5, 6);

        let mut subject = a;
        subject += &b;

        assert_eq!(subject.x(), 5);
        assert_eq!(subject.y(), 7);
        assert_eq!(subject.z(), 9);
    }
}

mod subtraction {
    use super::*;

    #[test]
    fn it_subtracts_the_components() {
        let a = Subject::new(5, 5, 5);
        let b = Subject::new(1, 2, 3);

        let subject = &a - &b;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 3);
        assert_eq!(subject.z(), 2);
    }

    #[test]
    fn it_can_mutate_the_normal() {
        let a = Subject::new(5, 5, 5);
        let b = Subject::new(1, 2, 3);

        let mut subject = a;
        subject -= &b;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 3);
        assert_eq!(subject.z(), 2);
    }
}

mod multiplication {
    use super::*;

    #[test]
    fn it_multiplies_by_a_scalar() {
        let subject = &Subject::new(1, 2, 3) * 4;

        assert_eq!(subject.x(), 4);
        assert_eq!(subject.y(), 8);
        assert_eq!(subject.z(), 12);
    }

    #[test]
    fn it_can_mutate_the_normal() {
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
        let subject = &Subject::new(1, 2, 3) / 10;

        assert_approx_eq!(subject.x(), 0.1);
        assert_approx_eq!(subject.y(), 0.2);
        assert_approx_eq!(subject.z(), 0.3);
    }

    #[test]
    fn it_can_mutate_the_normal() {
        let mut subject = Subject::new(1.0, 2.0, 3.0);

        subject /= 10;

        assert_approx_eq!(subject.x(), 0.1);
        assert_approx_eq!(subject.y(), 0.2);
        assert_approx_eq!(subject.z(), 0.3);
    }
}

mod negation {
    use super::*;

    #[test]
    fn it_returns_a_normal_pointing_in_the_opposite_direction() {
        let subject = -&Subject::new(1, 2, 3);

        assert_eq!(subject.x(), -1);
        assert_eq!(subject.y(), -2);
        assert_eq!(subject.z(), -3);
    }
}

mod abs {
    use super::*;

    #[test]
    fn it_can_take_the_absolute_value_of_normal3f_components() {
        let subject = Subject::new(-1.0, -2.0, -3.0).abs();

        assert_eq!(subject.x(), 1.0);
        assert_eq!(subject.y(), 2.0);
        assert_eq!(subject.z(), 3.0);
    }
}

mod dot {
    use super::*;

    #[test]
    fn it_returns_the_dot_product_of_two_normals() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(4, 5, 6);

        assert_eq!(a.dot(&b), 32);
    }

    #[test]
    fn it_returns_the_dot_product_of_the_normal_with_a_vector() {
        let normal = Subject::new(1, 2, 3);
        let vector = Vector3::new(4, 5, 6);

        assert_eq!(normal.dot(&vector), 32);
    }
}

mod abs_dot {
    use super::*;

    #[test]
    fn it_returns_the_absolute_value_of_the_dot_product_of_two_normals() {
        let a = Subject::new(1.0, 2.0, 3.0);
        let b = Subject::new(-4.0, -5.0, -6.0);

        assert_eq!(a.abs_dot(&b), 32.0);
    }

    #[test]
    fn it_returns_the_absolute_value_of_the_dot_product_with_a_vector() {
        let normal = Subject::new(1.0, 2.0, 3.0);
        let vector = Vector3::new(-4.0, -5.0, -6.0);

        assert_eq!(normal.abs_dot(&vector), 32.0);
    }
}

mod length_squared {
    use super::*;

    #[test]
    fn it_returns_the_square_of_the_length_of_the_normal() {
        let subject = Subject::new(1, 2, 3);

        assert_eq!(subject.length_squared(), 14.0);
    }
}

mod length {
    use super::*;

    #[test]
    fn it_returns_the_length_of_the_normal() {
        let subject = Subject::new(1, 2, 3);

        assert_eq!(subject.length(), f64::sqrt(14.0));
    }
}

mod normalize {
    use super::*;

    #[test]
    fn it_divides_each_component_by_the_normals_length() {
        let subject = Subject::new(1, 2, 3).normalize();
        let divisor = f64::sqrt(14.0);

        assert_eq!(subject.x(), 1.0 / divisor);
        assert_eq!(subject.y(), 2.0 / divisor);
        assert_eq!(subject.z(), 3.0 / divisor);
    }
}

mod face_forward {
    use super::*;

    #[test]
    fn it_ensures_the_normal_is_in_the_same_hemisphere_as_another_normal() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(-5, -5, -5);

        let subject = a.face_forward(&b);

        assert_eq!(subject.x(), -1);
        assert_eq!(subject.y(), -2);
        assert_eq!(subject.z(), -3);
    }

    #[test]
    fn it_ensures_the_normal_is_in_the_same_hemisphere_as_the_vector() {
        let normal = Subject::new(1, 2, 3);
        let vector = Vector3::new(-5, -5, -5);

        let subject = normal.face_forward(&vector);

        assert_eq!(subject.x(), -1);
        assert_eq!(subject.y(), -2);
        assert_eq!(subject.z(), -3);
    }
}
