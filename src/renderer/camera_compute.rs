use crate::{Camera, Matrix, Ray, Vert};

pub struct CameraCompute {
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform: Matrix,
}

impl CameraCompute {
    pub fn new(camera: &Camera) -> CameraCompute {
        let half_view = (camera.fov / 2.0).tan();
        let (width, height) = camera.dimensions();
        let aspect_ratio = width as f64 / height as f64;
        let (half_width, half_height) = if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2.0 / width as f64;
        let orientation = Matrix::orientation(&camera.to, &camera.from, &camera.up);
        let transform = (orientation * Matrix::translate(-camera.from.x, -camera.from.y, -camera.from.z)).inverse();
        CameraCompute {
            half_width,
            half_height,
            pixel_size,
            transform,
        }
    }

    pub fn ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let x_offset = (x + 0.5) * self.pixel_size;
        let y_offset = (y + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = self.transform.multiply_with_vert(&Vert::new(world_x, world_y, -1.0));
        let origin = self.transform.multiply_with_vert(&Vert::new(0.0, 0.0, 0.0));
        let direction = (pixel - origin.clone()).normalise();
        Ray::new(origin, direction)
    }
}
