use crate::{col_at_ray, CameraIterator, ComputedMesh, Light, Vert};
use std::sync::Arc;

pub struct WorldIterator {
    pub computed_meshes: Vec<Arc<ComputedMesh>>,
    pub lights: Vec<Arc<Light>>,
    pub camera_iter: CameraIterator,
}

impl Iterator for WorldIterator {
    type Item = Vert;

    fn next(&mut self) -> Option<Vert> {
        if let Some(ray) = self.camera_iter.next() {
            let col = col_at_ray(&ray, &self.lights, &self.computed_meshes);
            Some(col)
        } else {
            None
        }
    }
}
