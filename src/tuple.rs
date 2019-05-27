#![allow(dead_code)]

use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
pub struct Tuple(pub f32, pub f32, pub f32, pub f32);

impl Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, other: &Tuple) -> Tuple {
        return Tuple(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        );
    }
}

impl Add for &Tuple {
    type Output = Tuple;

    fn add(self, other: &Tuple) -> Tuple {
        return Tuple(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        );
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        let eplison = 0.00001;
        let diff = self - other;
        return diff.0.abs() < eplison
            && diff.1.abs() < eplison
            && diff.2.abs() < eplison
            && diff.3.abs() < eplison;
    }
}

pub fn is_point(mat: &Tuple) -> bool {
    return mat.3 == 1.0;
}

pub fn is_vector(mat: &Tuple) -> bool {
    return mat.3 == 0.0;
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    return Tuple(x, y, z, 1.0);
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    return Tuple(x, y, z, 0.0);
}
