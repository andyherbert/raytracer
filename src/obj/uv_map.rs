use crate::{Map, Vert};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UVMap {
    pub map: Arc<Map>,
    pub t1: Vert,
    pub t2: Vert,
    pub t3: Vert,
}

impl UVMap {
    pub fn colour_at(&self, u: f64, v: f64) -> Vert {
        let coord = self.t1.clone() + (self.t2.clone() - self.t1.clone()).multiply_by_scalar(u) + (self.t3.clone() - self.t1.clone()).multiply_by_scalar(v);
        let x = (coord.x * (self.map.width - 1) as f64).floor();
        let y = ((1.0 - coord.y) * (self.map.height - 1) as f64).floor();
        self.map.colour_at(x, y)
    }
}
