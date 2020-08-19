use crate::{CameraCompute, Ray};

pub struct CameraIterator {
    pub width: usize,
    pub compute: CameraCompute,
    pub x: usize,
    pub start_y: usize,
    pub end_y: usize,
}

impl Iterator for CameraIterator {
    type Item = Ray;

    fn next(&mut self) -> Option<Ray> {
        if self.start_y == self.end_y {
            None
        } else {
            let ray = self.compute.ray_for_pixel(self.x as f64, self.start_y as f64);
            self.x += 1;
            if self.x == self.width {
                self.x = 0;
                self.start_y += 1;
            }
            Some(ray)
        }
    }
}
