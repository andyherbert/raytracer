use crate::{CameraCompute, CameraIterator, Ray, Vert, PI};

#[derive(Clone)]
pub struct Camera {
    pub width: usize,
    pub height: usize,
    pub fov: f64,
    pub from: Vert,
    pub to: Vert,
    pub up: Vert,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: f64) -> Camera {
        Camera {
            width,
            height,
            fov,
            from: Vert::new(0.0, -5.0, -5.0),
            to: Vert::new(0.0, 0.0, 0.0),
            up: Vert::new(0.0, 1.0, 0.0),
        }
    }

    pub fn ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let compute = CameraCompute::new(self);
        compute.ray_for_pixel(x, y)
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_dimensions(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn part_iter(&self, start_y: usize, end_y: usize) -> CameraIterator {
        CameraIterator {
            width: self.width,
            compute: CameraCompute::new(self),
            x: 0,
            start_y,
            end_y,
        }
    }
}

impl IntoIterator for Camera {
    type Item = Ray;
    type IntoIter = CameraIterator;

    fn into_iter(self) -> CameraIterator {
        CameraIterator {
            width: self.width,
            compute: CameraCompute::new(&self),
            x: 0,
            start_y: 0,
            end_y: self.height,
        }
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(300, 300, PI / 2.5)
    }
}
