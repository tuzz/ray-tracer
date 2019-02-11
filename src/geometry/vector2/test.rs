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
