
use std::ops::Sub;

#[derive(Debug)]
struct Tuple(f32, f32, f32, f32);

impl Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, other: &Tuple) -> Tuple {
        return Tuple(self.0 - other.0,
                     self.1 - other.1,
                     self.2 - other.2,
                     self.3 - other.3
        );
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        let eplison = 0.00001;
        let diff = self - other;
        return diff.0.abs() < eplison &&
               diff.1.abs() < eplison &&
               diff.2.abs() < eplison &&
               diff.3.abs() < eplison;
    }
}



fn is_point(mat: &Tuple) -> bool {
    return mat.3 == 1.0;
}

fn is_vector(mat: &Tuple) -> bool {
    return mat.3 == 0.0;
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    return Tuple(x, y, z, 1.0);
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    return Tuple(x, y, z, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = Tuple(4.3, -4.2, 3.1, 1.0);

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
