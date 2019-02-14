use super::*;

type Subject<T> = Bounds3<T>;

mod new {
    use super::*;

    #[test]
    fn it_builds_an_empty_bounding_box_for_f64() {
        let subject = Subject::<f64>::new();

        assert_eq!(subject.p_min.x(), std::f64::MAX);
        assert_eq!(subject.p_max.y(), std::f64::MIN);
    }

    #[test]
    fn it_builds_an_empty_bounding_box_for_i32() {
        let subject = Subject::<i32>::new();

        assert_eq!(subject.p_min.y(), std::i32::MAX);
        assert_eq!(subject.p_max.z(), std::i32::MIN);
    }
}
