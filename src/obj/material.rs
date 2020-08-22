use crate::Vert;

#[derive(Clone)]
pub struct Material {
    pub colour: Vert,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            colour: Vert::rgb(190, 190, 190),
            ambient: 0.25,
            diffuse: 1.0,
            specular: 0.0,
            shininess: 0.0,
        }
    }
}
