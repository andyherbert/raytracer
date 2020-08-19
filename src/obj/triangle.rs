use crate::{ComputedTriangle, Material, Matrix, Vert};

#[derive(Clone)]
pub struct Triangle {
    pub p1: Vert,
    pub p2: Vert,
    pub p3: Vert,
    pub normals: Option<(Vert, Vert, Vert)>,
    pub material: Material,
}

impl Triangle {
    pub fn compute(&self) -> ComputedTriangle {
        ComputedTriangle::new(self.p1.clone(), self.p2.clone(), self.p3.clone(), self.normals.clone())
    }

    pub fn transform(&mut self, transform: &Matrix) {
        self.p1 = transform.multiply_with_vert(&self.p1);
        self.p2 = transform.multiply_with_vert(&self.p2);
        self.p3 = transform.multiply_with_vert(&self.p3);
    }

    pub fn transform_normals(&mut self, transform: &Matrix) {
        self.normals = match &self.normals {
            Some((n1, n2, n3)) => {
                let n1 = transform.multiply_with_vert(&n1);
                let n2 = transform.multiply_with_vert(&n2);
                let n3 = transform.multiply_with_vert(&n3);
                Some((n1, n2, n3))
            }
            None => None,
        };
    }
}

pub trait TransformTriangles {
    fn transform_triangles(&mut self, matrix: Matrix);
}

impl TransformTriangles for Vec<Triangle> {
    fn transform_triangles(&mut self, matrix: Matrix) {
        for triangle in self {
            triangle.transform(&matrix);
        }
    }
}

pub trait TransformNormals {
    fn transform_normals(&mut self, matrix: Matrix);
}

impl TransformNormals for Vec<Triangle> {
    fn transform_normals(&mut self, matrix: Matrix) {
        for triangle in self {
            triangle.transform_normals(&matrix);
        }
    }
}
