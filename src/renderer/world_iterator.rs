use crate::{CameraIterator, ComputedMesh, Light, Vert};
use super::colour_at_ray;
use std::sync::Arc;

pub struct WorldIterator {
    pub computed_meshes: Vec<Arc<ComputedMesh>>,
    pub lights: Vec<Arc<Light>>,
    pub camera_iter: CameraIterator,
}

impl Iterator for WorldIterator {
    type Item = Vert;

    fn next(&mut self) -> Option<Vert> {
        match self.camera_iter.next() {
            Some(ray) => {
                let col = colour_at_ray(&ray, &self.lights, &self.computed_meshes);
                Some(col)
            },
            None => None,
        }
    }
}
