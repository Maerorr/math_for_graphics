use std::{ops};

use crate::point::*;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    
    /// Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    /// create a vector that points from one point to another
    pub fn from_points(p1: &Point, p2: &Point) -> Vector {
        Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z)
    }

    /// dot product, multiplication of all components
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// cross product, result is a perpendicular vector
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    /// Returns the angle between two vectors in **radians**
    pub fn angle_radians(&self, other: &Vector) -> f64 {
        // get the dot product
        let dot = self.dot(other);
        // calculate lengths of both vectors
        let len1 = self.length();
        let len2 = other.length();
        // calculate the angle
        let cos = dot / (len1 * len2);
        // return the angle in radians
        cos.acos()
    }

    pub fn angle_degrees(&self, other: &Vector) -> f64 {
        self.angle_radians(other) * 180.0 / std::f64::consts::PI
    }

    /// Returns the length of a vector
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes a vector, which means it makes it's length equal to 1
    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    /// Converts a vector to a string and returns it
    pub fn to_string(&self) -> String {
        let out: String = format!("[{:.2}, {:.2}, {:.2}]", self.x, self.y, self.z);
        out
    }
}

// + operator overload
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// - operator overload
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// * operator overload
// scalar multiply ([vector] * scalar)
impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

// / operator overload
// vector scalar division ([vector] / scalar)
// in case of division by zero, return the original vector
impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, scalar: f64) -> Vector {
        if scalar == 0.0 {
            print!("Warning: division by zero. Vector values were not altered.");
            return self;
        } else
        {
            Vector {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
            }
        }
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, scalar: f64) {
        if scalar == 0.0 {
            print!("Warning: division by zero. Vector values were not altered.");
        } else
        {
            self.x /= scalar;
            self.y /= scalar;
            self.z /= scalar;
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
   