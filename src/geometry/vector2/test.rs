use super::*;

type Subject<T> = Vector2<T>;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_two_dimensional_vector() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject.x, 1);
        assert_eq!(subject.y, 2);
    }
}

mod index {
    use super::*;

    #[test]
    fn it_can_index_into_fields_of_the_struct() {
        let subject = Subject::new(1, 2);

        assert_eq!(subject[0], 1);
        assert_eq!(subject[1], 2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: 2")]
    fn it_panics_if_out_of_bounds() {
        let subject = Subject::new(1, 2);

        subject[2];
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
