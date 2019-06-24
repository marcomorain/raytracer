use crate::tuple;
use std::fmt::Write;

#[derive(Debug)]
pub struct Canvas {
    w: usize,
    h: usize,
    data: Vec<tuple::Tuple>,
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    let size: usize = width * height;
    let mut data = Vec::with_capacity(size);
    data.resize(size, tuple::ZERO);
    return Canvas {
        w: width,
        h: height,
        data: data,
    };
}

fn clamp(num: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    let mut x = num;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

fn color(c: f32) -> i32 {
    return clamp(255.0 * c, 0.0, 255.0).floor() as i32;
}
impl Canvas {
    pub fn write_pixel(&mut self, x: usize, y: usize, color: &tuple::Tuple) {
        self.data[(y * self.w) + x] = color.clone();
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &tuple::Tuple {
        return &self.data[(y * self.w) + x];
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        write!(&mut ppm, "P3\n{} {}\n255\n", self.w, self.h).unwrap();
        for y in 0..self.h {
            for x in 0..self.w {
                let pixel = &self.data[(self.w * y) + x];
                write!(
                    &mut ppm,
                    "{} {} {}",
                    color(pixel.0),
                    color(pixel.1),
                    color(pixel.2)
                )
                .unwrap();
            }
            ppm.push_str("\n");
        }
        ppm.push_str("\n");
        return ppm;
    }
}
