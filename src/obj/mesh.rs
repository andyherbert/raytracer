use crate::{
    ComputeTriangles, ComputedMesh, Map, Matrix, TransformNormals, TransformTriangles, Triangle,
    Vert,
};
use super::WavefrontObj;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub position: Vert,
    pub scale: Vert,
    pub rotation: Vert,
    pub maps: HashMap<String, Arc<Map>>,
}

impl Mesh {
    pub fn load_obj(path: &str) -> Result<Mesh, Box<dyn std::error::Error>> {
        let obj = Mesh::from(WavefrontObj::open(path)?);
        Ok(obj)
    }

    pub fn compute(&self) -> ComputedMesh {
        let mut triangles = self.triangles.clone();
        if self.rotation.x != 0.0 {
            let matrix = Matrix::rot_x(self.rotation.x);
            triangles.transform_triangles(matrix.clone());
            triangles.transform_normals(matrix);
        }
        if self.rotation.y != 0.0 {
            let matrix = Matrix::rot_y(self.rotation.y);
            triangles.transform_triangles(matrix.clone());
            triangles.transform_normals(matrix);
        }
        if self.rotation.z != 0.0 {
            let matrix = Matrix::rot_z(self.rotation.z);
            triangles.transform_triangles(matrix.clone());
            triangles.transform_normals(matrix);
        }
        if self.position.x != 0.0 || self.position.y != 0.0 || self.position.z != 0.0 {
            triangles.transform_triangles(Matrix::translate(self.position.x, self.position.y, self.position.z));
        }
        if self.scale.x != 0.0 || self.scale.y != 0.0 || self.scale.z != 0.0 {
            triangles.transform_triangles(Matrix::scale(self.scale.x, self.scale.y, self.scale.z));
        }
        let triangles = triangles.compute_triangles();
        ComputedMesh {triangles}
    }
}
