use crate::{Material, Ray, Triangle, Vert, MACHEPS};

pub struct ComputedTriangle {
    p1: Vert,
    pub normals: Option<(Vert, Vert, Vert)>,
    e1: Vert,
    e2: Vert,
    pub norm_v: Vert,
    pub material: Material,
}

impl ComputedTriangle {
    pub fn new(p1: Vert, p2: Vert, p3: Vert, normals: Option<(Vert, Vert, Vert)>) -> ComputedTriangle {
        let e1 = p2.clone() - p1.clone();
        let e2 = p3.clone() - p1.clone();
        let norm_v = e2.cross_product(&e1).normalise();
        ComputedTriangle {
            p1,
            normals,
            e1,
            e2,
            norm_v,
            material: Material::default(),
        }
    }

    pub fn intersects_with(&self, ray: &Ray) -> Option<(f64, f64, f64)> {
        let dir_cross_e2 = ray.direction.cross_product(&self.e2);
        let det = self.e1.dot_product(&dir_cross_e2);
        if det.abs() < MACHEPS {
            return None;
        }
        let f = 1.0 / det;
        let p1_to_origin = ray.origin.clone() - self.p1.clone();
        let u = f * p1_to_origin.dot_product(&dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let origin_cross_e1 = p1_to_origin.cross_product(&self.e1);
        let v = f * ray.direction.dot_product(&origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }
        let time = f * self.e2.dot_product(&origin_cross_e1);
        if time < 0.0 {
            return None;
        }
        Some((time, u, v))
    }

    pub fn norm_vec_at_uv(&self, u: f64, v: f64) -> Vert {
        if let Some((n1, n2, n3)) = &self.normals {
            n2.multiply_by_scalar(u) + n3.multiply_by_scalar(v) + n1.multiply_by_scalar(1.0 - u - v)
        } else {
            self.norm_v.clone()
        }
    }

    pub fn colour_at_uv(&self, _u: f64, _v: f64) -> Vert {
        self.material.colour.clone()
    }
}

pub trait ComputeTriangles {
    fn compute_triangles(&self) -> Vec<ComputedTriangle>;
}

impl ComputeTriangles for Vec<Triangle> {
    fn compute_triangles(&self) -> Vec<ComputedTriangle> {
        self.iter().map(|triangle| triangle.compute()).collect()
    }
}
