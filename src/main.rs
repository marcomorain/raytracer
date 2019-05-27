#![allow(dead_code)]

mod tuple;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = tuple::Tuple(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.0, 4.3);
        assert_eq!(a.1, -4.2);
        assert_eq!(a.2, 3.1);
        assert_eq!(a.3, 1.0);

        assert!(is_point(&a));
        assert!(!is_vector(&a));
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = Tuple(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.0, 4.3);
        assert_eq!(a.1, -4.2);
        assert_eq!(a.2, 3.1);
        assert_eq!(a.3, 0.0);

        assert!(!is_point(&a));
        assert!(is_vector(&a));
}

    #[test]
    fn point_creates_tuples_with_w_1 () {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuples_with_w_0 () {
        let p = vector(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple(4.0, -4.0, 3.0, 0.0));
    }

}

fn main() {
    println!("Go speedracer!");
}
