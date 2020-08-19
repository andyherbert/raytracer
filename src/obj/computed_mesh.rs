use crate::{ComputedTriangle, Intersection, Mesh, Ray};
use std::sync::Arc;

pub struct ComputedMesh {
    pub triangles: Vec<ComputedTriangle>,
}

impl ComputedMesh {
    pub fn intersects_with<'a>(&'a self, ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        for triangle in &self.triangles {
            if let Some((time, u, v)) = triangle.intersects_with(&ray) {
                let intersection = Intersection {
                    time,
                    u,
                    v,
                    triangle,
                };
                intersections.push(intersection);
            }
        }
    }
}

pub trait ComputeMeshes {
    fn compute_meshes(&self) -> Vec<Arc<ComputedMesh>>;
}

impl ComputeMeshes for Vec<Arc<Mesh>> {
    fn compute_meshes(&self) -> Vec<Arc<ComputedMesh>> {
        self.iter().map(|mesh| Arc::new(mesh.compute())).collect()
    }
}
