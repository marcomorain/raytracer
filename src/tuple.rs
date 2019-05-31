#![allow(dead_code)]

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
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

impl Neg for &Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        return Tuple(-self.0, -self.1, -self.2, -self.3);
    }
}

impl Mul<f32> for &Tuple {
    type Output = Tuple;
    fn mul(self, other: f32) -> Tuple {
        return Tuple(
            self.0 * other,
            self.1 * other,
            self.2 * other,
            self.3 * other,
        );
    }
}

impl Div<f32> for &Tuple {
    type Output = Tuple;
    fn div(self, other: f32) -> Tuple {
        return Tuple(
            self.0 / other,
            self.1 / other,
            self.2 / other,
            self.3 / other,
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

impl Tuple {
    pub fn is_point(&self) -> bool {
        return self.3 == 1.0;
    }

    pub fn is_vector(&self) -> bool {
        return self.3 == 0.0;
    }

    pub fn magnitude(&self) -> f32 {
        return ((self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2) + (self.3 * self.3))
            .sqrt();
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        return Tuple(self.0 / mag, self.1 / mag, self.2 / mag, self.3 / mag);
    }

    pub fn dot(&self, other: &Tuple) -> f32 {
        return self.0 * other.0
            + self.1 * other.1
            + self.2 * other.2
            + self.3 * other.3;
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        return vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        );
    }
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    return Tuple(x, y, z, 1.0);
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    return Tuple(x, y, z, 0.0);
}
