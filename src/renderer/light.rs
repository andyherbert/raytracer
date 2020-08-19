use crate::{ComputedMesh, Ray, Vert, IntersectionCompute};
use std::sync::Arc;

#[derive(Clone)]
pub struct Light {
    pub position: Vert,
    pub intensity: Vert,
    pub casts_shadows: bool,
}

impl Light {
    pub fn new() -> Light {
        Light {
            position: Vert::new(0.0, 0.0, 0.0),
            intensity: Vert::rgb(255, 255, 255),
            casts_shadows: true,
        }
    }

    pub fn lighting(&self, compute: &IntersectionCompute, shadowed: bool) -> Vert {
        let effective_colour = self.intensity.clone() * compute.colour.clone();
        let light_v = (compute.point.clone() - self.position.clone()).normalise();
        let ambient = effective_colour.multiply_by_scalar(compute.triangle.material.ambient);
        let light_dot_normal = light_v.dot_product(&compute.norm_v);
        let (diffuse, specular) = if shadowed || light_dot_normal < 0.0 {
            let diffuse = Vert::black();
            let specular = Vert::black();
            (diffuse, specular)
        } else {
            let diffuse = effective_colour.multiply_by_scalar(compute.triangle.material.diffuse).multiply_by_scalar(light_dot_normal);
            let light_v = (compute.point.clone() - self.position.clone()).normalise();
            let reflect_v = light_v.clone() - compute.norm_v.multiply_by_scalar(2.0).multiply_by_scalar(light_v.dot_product(&compute.norm_v));
            let reflect_dot_eye = reflect_v.dot_product(&compute.eye_v);
            if reflect_dot_eye <= 0.0 {
                (diffuse, Vert::black())
            } else {
                let factor = reflect_dot_eye.powf(compute.triangle.material.shininess);
                let specular = self.intensity.multiply_by_scalar(compute.triangle.material.specular * factor);
                (diffuse, specular)
            }
        };
        ambient + diffuse + specular
    }

    pub fn shadowed(&self, over_point: Vert, meshes: &Vec<Arc<ComputedMesh>>) -> bool {
        let vec = self.position.clone() - over_point.clone();
        let distance = vec.magnitude();
        let direction = vec.normalise();
        let ray = Ray::new(over_point, direction);
        for mesh in meshes {
            for triangle in &mesh.triangles {
                if let Some((time, ..)) = triangle.intersects_with(&ray) {
                    if time < distance {
                        return true;
                    }
                }
            }
        }
        false
    }
}
