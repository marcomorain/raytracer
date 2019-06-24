use crate::tuple;

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

impl Canvas {
    pub fn write_pixel(&mut self, x: usize, y: usize, color: &tuple::Tuple) {
        self.data[(y * self.w) + x] = color.clone();
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &tuple::Tuple {
        return &self.data[(y * self.w) + x];
    }
}
