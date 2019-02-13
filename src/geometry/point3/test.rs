use super::*;

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
