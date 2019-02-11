use super::*;
use assert_approx_eq::assert_approx_eq;

type Subject<T> = Vector3<T>;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_three_dimensional_vector() {
        let subject = Subject::new(1, 2, 3);

        assert_eq!(subject.x, 1);
        assert_eq!(subject.y, 2);
        assert_eq!(subject.z, 3);
    }
}

mod index {
    use super::*;

    #[test]
    fn it_can_index_into_fields_of_the_struct() {
        let subject = Subject::new(1, 2, 3);

        assert_eq!(subject[0], 1);
        assert_eq!(subject[1], 2);
        assert_eq!(subject[2], 3);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: 3")]
    fn it_panics_if_out_of_bounds() {
        let subject = Subject::new(1, 2, 3);

        subject[3];
    }
}

mod aliases {
    use super::*;

    #[test]
    fn it_has_a_type_alias_for_a_vector_of_signed_integers() {
        Vector3i::new(-1 as i32, 2, -3);
    }

    #[test]
    fn it_has_a_type_alias_for_a_vector_of_double_precision_floats() {
        Vector3f::new(0.1 as f64, 0.2, 0.3);
    }
}

mod default {
    use super::*;

    #[test]
    fn it_sets_x_y_and_z_to_zero() {
        let subject = Subject::<u32>::default();

        assert_eq!(subject.x, 0);
        assert_eq!(subject.y, 0);
        assert_eq!(subject.z, 0);
    }
}

mod addition {
    use super::*;

    #[test]
    fn it_adds_the_components() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(4, 5, 6);

        let subject = a + b;

        assert_eq!(subject.x, 5);
        assert_eq!(subject.y, 7);
        assert_eq!(subject.z, 9);
    }

    #[test]
    fn it_can_mutate_the_vector() {
        let a = Subject::new(1, 2, 3);
        let b = Subject::new(4, 5, 6);

        let mut subject = a;
        subject += b;

        assert_eq!(subject.x, 5);
        assert_eq!(subject.y, 7);
        assert_eq!(subject.z, 9);
    }
}

mod subtraction {
    use super::*;

    #[test]
    fn it_subtracts_the_components() {
        let a = Subject::new(5, 5, 5);
        let b = Subject::new(1, 2, 3);

        let subject = a - b;

        assert_eq!(subject.x, 4);
        assert_eq!(subject.y, 3);
        assert_eq!(subject.z, 2);
    }

    #[test]
    fn it_can_mutate_the_vector() {
        let a = Subject::new(5, 5, 5);
        let b = Subject::new(1, 2, 3);

        let mut subject = a;
        subject -= b;

        assert_eq!(subject.x, 4);
        assert_eq!(subject.y, 3);
        assert_eq!(subject.z, 2);
    }
}

mod multiplication {
    use super::*;

    #[test]
    fn it_multiplies_by_a_scalar() {
        let subject = Subject::new(1, 2, 3) * 4;

        assert_eq!(subject.x, 4);
        assert_eq!(subject.y, 8);
        assert_eq!(subject.z, 12);
    }

    #[test]
    fn it_can_mutate_the_vector() {
        let mut subject = Subject::new(1, 2, 3);

        subject *= 4;

        assert_eq!(subject.x, 4);
        assert_eq!(subject.y, 8);
        assert_eq!(subject.z, 12);
    }
}

mod division {
    use super::*;

    #[test]
    fn it_divides_by_a_divisor() {
        let subject = Subject::new(1, 2, 3) / 10;

        assert_approx_eq!(subject.x, 0.1);
        assert_approx_eq!(subject.y, 0.2);
        assert_approx_eq!(subject.z, 0.3);
    }

    #[test]
    fn it_can_mutate_the_vector() {
        let mut subject = Subject::new(1.0, 2.0, 3.0);

        subject /= 10;

        assert_approx_eq!(subject.x, 0.1);
        assert_approx_eq!(subject.y, 0.2);
        assert_approx_eq!(subject.z, 0.3);
    }
}
