use super::*;

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
