use crate::{ComputedTriangle, Intersection, Ray, Vert, MACHEPS};

pub struct IntersectionCompute<'a> {
    pub time: f64,
    pub triangle: &'a ComputedTriangle,
    pub point: Vert,
    pub norm_v: Vert,
    pub colour: Vert,
    pub over_point: Vert,
    pub eye_v: Vert,
}

impl<'a> IntersectionCompute<'a> {
    pub fn new(ray: &Ray, intersection: &'a Intersection) -> IntersectionCompute<'a> {
        let point = ray.pos(intersection.time);
        let norm_v = intersection.triangle.norm_vec_at_uv(intersection.u, intersection.v);
        let colour = intersection.triangle.colour_at_uv(intersection.u, intersection.v);
        let over_point = point.clone() - norm_v.multiply_by_scalar(MACHEPS);
        let eye_v = -ray.direction.clone();
        IntersectionCompute {
            time: intersection.time,
            triangle: intersection.triangle,
            point,
            norm_v,
            colour,
            over_point,
            eye_v,
        }
    }
}
