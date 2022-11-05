use std::f64::EPSILON;
use crate::{vector::*, math::*};

// used for partial_eq
use float_cmp::{approx_eq, F64Margin};

// 4x4 matrix of f64, column-major
#[derive(Debug, Clone)]
pub struct Mat4 {
    pub m: [[f64; 4]; 4],
}

impl Mat4 {
    pub fn new() -> Mat4 {
        Mat4 {
            m: [[0.0; 4]; 4],
        }
    }

    // create an identity matrix
    pub fn identity() -> Mat4 {
        Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // multiply this by other mat4 matrix
    pub fn multiply(&mut self, other: &Mat4) {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        self.m = result.m;
    }

    // translate the matrix by a vector
    pub fn translate(&mut self, Vector { x, y, z }: Vector) {
        self.m[0][3] += x;
        self.m[1][3] += y;
        self.m[2][3] += z;
    }

    // scales the matrix by a vector
    pub fn scale(&mut self, Vector { x, y, z }: Vector) {
        self.m[0][0] *= x;
        self.m[1][1] *= y;
        self.m[2][2] *= z;
    }

    // for information about this algorithm, see:
    // https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle
    pub fn rotate(&mut self, angle: f64, Vector { x, y, z }: Vector) {
        let mut result = Mat4::new();
        let mut axis = Vector::new(x, y, z);

        axis.normalize();

        let sin = (angle as f64).sin();
        let cos = (angle as f64).cos();
        let t = 1.0 - cos;

        result.m[0][0] = t * axis.x * axis.x + cos;
        result.m[0][1] = t * axis.x * axis.y - sin * axis.z;
        result.m[0][2] = t * axis.x * axis.z + sin * axis.y;

        result.m[1][0] = t * axis.x * axis.y + sin * axis.z;
        result.m[1][1] = t * axis.y * axis.y + cos;
        result.m[1][2] = t * axis.y * axis.z - sin * axis.x;

        result.m[2][0] = t * axis.x * axis.z - sin * axis.y;
        result.m[2][1] = t * axis.y * axis.z + sin * axis.x;
        result.m[2][2] = t * axis.z * axis.z + cos;

        result.m[3][3] = 1.0;

        self.multiply(&result);
    }

    // simple to_string for debugging purposes
    pub fn to_string(&self) -> String {
        let mut out: String = String::new();
        for i in 0..4 {
            out.push_str(&format!(
                "[{:.2} {:.2} {:.2} {:.2}]\n",
                self.m[i][0], self.m[i][1], self.m[i][2], self.m[i][3]
            ));
        }
        out
    }

}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Mat4) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                // using comparison with epsilon + units of least precision
                if !approx_eq!(f64, self.m[i][j], other.m[i][j], F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
                    return false;
                }
            }
        }
        true
    }
}


// ###########
// ## TESTS ##
// ###########

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identity() {
        let m = Mat4::identity();
        let identity = Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, identity);
    }

    #[test]
    fn test_multiply() {
        let mut m = Mat4::identity();

        m.scale(Vector::new(3.0, 2.0, 5.0));
        m.translate(Vector::new(1.0, 2.0, 3.0));

        let mut m2 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ],
        };

        m.multiply(&m2);

        let result = Mat4 {
            m: [
                [16.0, 20.0, 24.0, 28.0],
                [36.0, 40.0, 44.0, 48.0],
                [84.0, 92.0, 100.0, 108.0],
                [13.0, 14.0, 15.0, 16.0],
            ],
        };

        assert_eq!(m, result);
    }

    #[test]
    fn test_translate() {
        let mut m = Mat4::identity();
        m.translate(Vector::new(1.0, 2.0, 3.0));
        let result = Mat4 {
            m: [
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 2.0],
                [0.0, 0.0, 1.0, 3.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, result);
    }

    #[test]
    fn test_scale() {
        let mut m = Mat4::identity();
        m.scale(Vector::new(1.0, 2.0, 3.0));
        let result = Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 3.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, result);
    }

    #[test]
    fn test_rotate() {
        let mut m = Mat4::identity();
        m.rotate(as_radians(90.0), Vector::new(0.0, 0.0, 1.0));
        let result = Mat4 {
            m: [
                [0.0, -1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(m, result);
    }
}