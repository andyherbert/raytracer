#[derive(Debug, Default, Clone)]
pub struct Vert {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vert {
    pub fn new(x: f64, y: f64, z: f64) -> Vert {
        Vert {x, y, z}
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Vert {
        Vert {
            x: r as f64 / 255.0,
            y: g as f64 / 255.0,
            z: b as f64 / 255.0,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalise(self) -> Vert {
        let magnitude = self.magnitude();
        Vert {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn cross_product(&self, other: &Vert) -> Vert {
        Vert {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot_product(&self, other: &Vert) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn multiply_by_scalar(&self, scalar: f64) -> Vert {
        Vert {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl core::ops::Sub for Vert {
    type Output = Vert;
    fn sub(self, other: Vert) -> Vert {
        Vert {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl core::ops::Add for Vert {
    type Output = Vert;
    fn add(self, other: Vert) -> Vert {
        Vert {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl core::ops::Mul for Vert {
    type Output = Vert;
    fn mul(self, other: Vert) -> Vert {
        Vert {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl core::ops::Neg for Vert {
    type Output = Vert;
    fn neg(self) -> Vert {
        Vert {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<Vert> for [u8; 4] {
    fn from(vec: Vert) -> [u8; 4] {
        let r = (vec.x.abs() * 255.0).floor() as u8;
        let g = (vec.y.abs() * 255.0).floor() as u8;
        let b = (vec.z.abs() * 255.0).floor() as u8;
        let a = 255;
        [r, g, b, a]
    }
}
