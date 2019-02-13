use assert_approx_eq::assert_approx_eq;
use super::*;

type Subject = CoordinateSystem;

fn assert_vector_eq(a: Vector3f, b: Vector3f) {
    assert_approx_eq!(a.x(), b.x());
    assert_approx_eq!(a.y(), b.y());
    assert_approx_eq!(a.z(), b.z());
}

mod new {
    use super::*;

    #[test]
    fn it_builds_a_coordinate_system_by_calculating_three_orthogonal_vectors() {
        let vector = Vector3f::new(0.1, 0.2, 0.3).normalize();
        let subject = Subject::new(&vector);

        let expected_v1 = vector;
        let expected_v2 = Vector3f::new(0.0, 0.3, -0.2).normalize();
        let expected_v3 = expected_v1.cross(&expected_v2);

        assert_vector_eq(subject.v1, expected_v1);
        assert_vector_eq(subject.v2, expected_v2);
        assert_vector_eq(subject.v3, expected_v3);
    }
}
