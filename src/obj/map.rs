use crate::Vert;
use image::open;

#[derive(Clone, Debug)]
pub struct Map {
    image: Vec<Vert>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn open(path: &str) -> Result<Map, Box<dyn std::error::Error>> {
        let buffer = open(path)?.to_rgba();
        let width = buffer.width() as i32;
        let height = buffer.height() as i32;
        let rgba = buffer.to_vec();
        let mut image = vec![];
        for index in (0..rgba.len()).step_by(4) {
            let r = rgba[index + 0];
            let g = rgba[index + 1];
            let b = rgba[index + 2];
            image.push(Vert::rgb(r, g, b));
        }
        Ok(Map {image, width, height})
    }

    pub fn colour_at(&self, x: f64, y: f64) -> Vert {
        let x = x.min(self.width as f64 - 1.0).max(0.0) as i32;
        let y = y.min(self.height as f64 - 1.0).max(0.0) as i32;
        let index = y * self.width + x;
        self.image[index as usize].clone()
    }
}
