use crate::{ComputedTriangle, Matrix, Vert};

pub struct Ray {
    pub origin: Vert,
    pub direction: Vert,
}

impl Ray {
    pub fn new(origin: Vert, direction: Vert) -> Ray {
        Ray {origin, direction}
    }

    pub fn pos(&self, time: f64) -> Vert {
        self.origin.clone() + self.direction.multiply_by_scalar(time)
    }

    pub fn transform(&self, matrix: Matrix) -> Ray {
        Ray {
            origin: matrix.multiply_with_vert(&self.origin),
            direction: matrix.multiply_with_vert(&self.direction),
        }
    }
}

pub trait SortByTime {
    fn sort_by_time(&mut self);
}

impl SortByTime for Vec<Intersection<'_>> {
    fn sort_by_time(&mut self) {
        self.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
    }
}

pub struct Intersection<'a> {
    pub time: f64,
    pub u: f64,
    pub v: f64,
    pub triangle: &'a ComputedTriangle,
}

impl<'a> Intersection<'a> {
    pub fn new(time: f64, u: f64, v: f64, triangle: &'a ComputedTriangle) -> Intersection<'a> {
        Intersection {
            time,
            u,
            v,
            triangle,
        }
    }
}
