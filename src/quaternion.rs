use std::ops;
use float_cmp::{approx_eq, F64Margin};
use crate::mat4::Mat4;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub real: f64,
    pub ivec: Vector,
}

impl Quaternion {
    pub fn new(real: f64, ivec: Vector) -> Quaternion {
        Quaternion { real, ivec }
    }

    pub fn identity() -> Quaternion {
        Quaternion::new(1.0, Vector::new(0.0, 0.0, 0.0))
    }

    pub fn rotate(&mut self, angle: f64, Vector { x, y, z, w}: Vector) {
        let angle = angle * 0.5;
        let sin = angle.sin();
        let cos = angle.cos();
        let q = Quaternion::new(cos, Vector::new(x * sin, y * sin, z * sin));
        *self = *self * q;
    }

    // Using the formula P' = H(H(R, P), R*) where H is the Hamilton product and R* is the conjugate of R
    pub fn rotate_vec(&self, vec: &mut Vector) {
        let q1 = self.hamilton_product(&Quaternion::new(0.0, *vec));
        let mut self_inv = self.clone();
        self_inv.conjugate();
        let q2 = q1.hamilton_product(&self_inv);
        *vec = q2.ivec;
    }

    pub fn hamilton_product(&self, other: &Quaternion) -> Quaternion {
        let Quaternion { real: a, ivec: Vector { x: i, y: j, z: k, w: _ } } = *self;
        let Quaternion { real: b, ivec: Vector { x: l, y: m, z: n, w: _ } } = *other;
        Quaternion::new(
            a * b - i * l - j * m - k * n,
            Vector::new(
                a * l + i * b + j * n - k * m,
                a * m + j * b + k * l - i * n,
                a * n + k * b + i * m - j * l,
            ),
        )
    }

    pub fn to_mat4(&self) -> Mat4 {
        let mut mat = Mat4::new();
        let q1 = self.real;
        let q2 = self.ivec.x;
        let q3 = self.ivec.y;
        let q4 = self.ivec.z;

        mat.m[0][0] = 1.0 - 2.0 * q3 * q3 - 2.0 * q4 * q4;
        mat.m[0][1] = 2.0 * q2 * q3 - 2.0 * q1 * q4;
        mat.m[0][2] = 2.0 * q2 * q4 + 2.0 * q1 * q3;
        //mat[0][3] = 0.0;
        mat.m[1][0] = 2.0 * q2 * q3 + 2.0 * q1 * q4;
        mat.m[1][1] = 1.0 - 2.0 * q2 * q2 - 2.0 * q4 * q4;
        mat.m[1][2] = 2.0 * q3 * q4 - 2.0 * q1 * q2;
        //mat[1][3] = 0.0;
        mat.m[2][0] = 2.0 * q2 * q4 - 2.0 * q1 * q3;
        mat.m[2][1] = 2.0 * q3 * q4 + 2.0 * q1 * q2;
        mat.m[2][2] = 1.0 - 2.0 * q2 * q2 - 2.0 * q3 * q3;
        //mat[2][3] = 0.0;

        mat
    }

    pub fn inverse(&mut self) {
        let mut quat = Quaternion::new(self.real, self.ivec * -1.0);
        let inv = 1.0 / (self.real.powi(2) * self.ivec.dot(&self.ivec));
        if approx_eq!(f64, inv, 0.0, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
            print!("Warning: division by zero. Quaternion values were not altered.");
            return;
        }
        quat *= inv;

        self.real = quat.real;
        self.ivec = quat.ivec;
    }

    pub fn normalize(&mut self) {
        let inv = 1.0 / self.ivec.length();
        self.real *= inv;
        self.ivec *= inv;
    }

    pub fn conjugate(&mut self) {
        self.ivec *= -1.0;
    }

    pub fn dot(&mut self, other: &Quaternion) -> f64 {
        (self.real * other.real) + self.ivec.dot(&other.ivec)
    }

    pub fn to_string(&self) -> String {
        let out: String = format!("({:.2}, {:.2}, {:.2}, {:.2})", self.real, self.ivec.x, self.ivec.y, self.ivec.z);
        out
    }
}

// OPERATOR OVERLOADS
// + operator overload
impl ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            real: self.real + other.real,
            ivec: self.ivec + other.ivec,
        }
    }
}

// += operator overload
impl ops::AddAssign<Quaternion> for Quaternion {
    fn add_assign(&mut self, other: Quaternion) {
        self.real += other.real;
        self.ivec += other.ivec;
    }
}

// - operator overload
impl ops::Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            real: self.real - other.real,
            ivec: self.ivec - other.ivec,
        }
    }
}

// -= operator overload
impl ops::SubAssign<Quaternion> for Quaternion {
    fn sub_assign(&mut self, other: Quaternion) {
        self.real -= other.real;
        self.ivec -= other.ivec;
    }
}

// * operator overload
// scalar * quaternion
impl ops::Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, scalar: f64) -> Quaternion {
        Quaternion {
            real: self.real * scalar,
            ivec: self.ivec * scalar,
        }
    }
}

// *= operator overload
// quaternion *= scalar
impl ops::MulAssign<f64> for Quaternion {
    fn mul_assign(&mut self, scalar: f64) {
        self.real *= scalar;
        self.ivec *= scalar;
    }
}

// quaternion * quaternion
impl ops::Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        let real_out = self.real * other.real - self.ivec.dot(&other.ivec);
        let ivec_out = self.ivec.cross(&other.ivec) + self.ivec * other.real + other.ivec * self.real;

        Quaternion {
            real: real_out,
            ivec: ivec_out,
        }
    }
}

// *= operator overload
impl ops::MulAssign for Quaternion {
    fn mul_assign(&mut self, other: Quaternion) {
        let real_out = self.real * other.real - self.ivec.dot(&other.ivec);
        let ivec_out = self.ivec.cross(&other.ivec) + self.ivec * other.real + other.ivec * self.real;

        self.real = real_out;
        self.ivec = ivec_out;
    }
}

// / operator overload
impl ops::Div for Quaternion {
    type Output = Quaternion;

    fn div(self, other: Quaternion) -> Quaternion {
        let out_quat = Quaternion::new(self.real, self.ivec);
        let mut other_inv = other.clone();
        other_inv.inverse();
        let out = out_quat * other_inv;
        Quaternion {
            real: out.real,
            ivec: out.ivec,
        }
    }
}

// /= operator overload
impl ops::DivAssign for Quaternion {
    fn div_assign(&mut self, other: Quaternion) {
        let out_quat = Quaternion::new(self.real, self.ivec);
        let mut other_inv = other.clone();
        other_inv.inverse();
        let out = out_quat * other_inv;
        self.real = out.real;
        self.ivec = out.ivec;
    }
}

// / operator overload
// quaternion / scalar
impl ops::Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, scalar: f64) -> Quaternion {
        Quaternion {
            real: self.real / scalar,
            ivec: self.ivec / scalar,
        }
    }
}

// /= operator overload
impl ops::DivAssign<f64> for Quaternion {
    fn div_assign(&mut self, scalar: f64) {
        self.real /= scalar;
        self.ivec /= scalar;
    }
}


#[cfg(test)]
mod test {
    use crate::math::as_radians;
    use super::*;

    #[test]
    // Check if the rotations are correctly applied and then converted to rotation matrix
    fn to_mat_test() {
        let mut quat = Quaternion::identity();
        quat.rotate(as_radians(90.0), Vector::new(1.0, 0.0, 0.0));
        let quat_mat = quat.to_mat4();

        let mut other_mat = Mat4::identity();
        other_mat.rotate(as_radians(90.0), Vector::new(1.0, 0.0, 0.0));

        let (mut vec1, mut vec2) = (Vector::new(1.0, 1.0, 1.0), Vector::new(1.0, 1.0, 1.0));
        vec1 = vec1 * quat_mat;
        vec2 = vec2 * other_mat;
        assert_eq!(vec1, vec2);
    }
}