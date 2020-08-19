use crate::Vert;

#[derive(Clone)]
pub struct Matrix {
    pub size: usize,
    pub data: [f64; 16],
}

impl Matrix {
    pub fn new(size: usize) -> Matrix {
        Matrix {
            size,
            data: [
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ],
        }
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
        Matrix {
            size: 4,
            data: [
                1.0, 0.0, 0.0, x,
                0.0, 1.0, 0.0, y,
                0.0, 0.0, 1.0, z,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
        Matrix {
            size: 4,
            data: [
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rot_x(radians: f64) -> Matrix {
        Matrix {
            size: 4,
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, radians.cos(), -radians.sin(), 0.0,
                0.0, radians.sin(), radians.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rot_y(radians: f64) -> Matrix {
        Matrix {
            size: 4,
            data: [
                radians.cos(), 0.0, radians.sin(), 0.0,
                0.0, 1.0, 0.0, 0.0,
                -radians.sin(), 0.0, radians.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rot_z(radians: f64) -> Matrix {
        Matrix {
            size: 4,
            data: [
                radians.cos(), -radians.sin(), 0.0, 0.0,
                radians.sin(), radians.cos(), 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn orientation(to: &Vert, from: &Vert, up: &Vert) -> Matrix {
        let forward = (to.clone() - from.clone()).normalise();
        let upn = up.clone().normalise();
        let left = forward.cross_product(&upn);
        let true_up = left.cross_product(&forward);
        Matrix {
            size: 4,
            data: [
                left.x, left.y, left.z, 0.0,
                true_up.x, true_up.y, true_up.z, 0.0,
                -forward.x, -forward.y, -forward.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn multiply_with_vert(&self, point: &Vert) -> Vert {
        Vert {
            x: self.data[0] * point.x + self.data[1] * point.y + self.data[2] * point.z + self.data[3],
            y: self.data[4] * point.x + self.data[5] * point.y + self.data[6] * point.z + self.data[7],
            z: self.data[8] * point.x + self.data[9] * point.y + self.data[10] * point.z + self.data[11],
        }
    }

    pub fn transpose(self) -> Matrix {
        let mut transposed = Matrix::new(4);
        let mut src_index = 0;
        for row in 0..self.size as usize {
            for column in 0..self.size as usize {
                transposed.data[src_index] = self.data[column * self.size as usize + row];
                src_index += 1;
            }
        }
        transposed
    }

    pub fn determinant(&self) -> f64 {
        match self.size {
            2 => self.data[0] * self.data[3] - self.data[1] * self.data[2],
            3 => {
                self.data[0] * self.co_factor(0, 0)
                    + self.data[1] * self.co_factor(0, 1)
                    + self.data[2] * self.co_factor(0, 2)
            }
            4 => {
                self.data[0] * self.co_factor(0, 0)
                    + self.data[1] * self.co_factor(0, 1)
                    + self.data[2] * self.co_factor(0, 2)
                    + self.data[3] * self.co_factor(0, 3)
            }
            _ => panic!(),
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix {
        let mut answer = match self.size {
            4 => Matrix::new(3),
            3 => Matrix::new(2),
            _ => panic!(),
        };
        let mut src_index = 0;
        let mut dest_index = 0;
        for src_row in 0..self.size {
            for src_column in 0..self.size {
                if src_row != row && src_column != column {
                    answer.data[dest_index] = self.data[src_index];
                    dest_index += 1;
                }
                src_index += 1;
            }
        }
        answer
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn co_factor(&self, row: usize, column: usize) -> f64 {
        let min = self.minor(row, column);
        match (row % 2, column % 2) {
            (0, 0) | (1, 1) => min,
            _ => -min,
        }
    }

    pub fn inverse(&self) -> Matrix {
        let mut answer = match self.size {
            4 => Matrix::new(4),
            3 => Matrix::new(3),
            2 => Matrix::new(2),
            _ => panic!(),
        };
        let det = self.determinant();
        if det == 0.0 {
            for i in 0..self.size.pow(2) as usize {
                answer.data[i] = self.data[i];
            }
            answer
        } else {
            let mut index = 0;
            for row in 0..self.size {
                for column in 0..self.size {
                    answer.data[index] = self.co_factor(row, column) / det;
                    index += 1;
                }
            }
            answer.transpose()
        }
    }
}

impl core::ops::Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        let mut answer = Matrix::new(4);
        for row_start in (0..16).step_by(4) {
            for column in 0..4 {
                let answer_index = row_start + column;
                answer.data[answer_index] += self.data[row_start + 0] * other.data[0 + column];
                answer.data[answer_index] += self.data[row_start + 1] * other.data[4 + column];
                answer.data[answer_index] += self.data[row_start + 2] * other.data[8 + column];
                answer.data[answer_index] += self.data[row_start + 3] * other.data[12 + column];
            }
        }
        answer
    }
}
