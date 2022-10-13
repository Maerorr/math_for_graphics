use std::{ops, fmt, string};

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
    pub fn angle(&self, other: &Vector) -> f64 {
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

    /// Returns the length of a vector
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes a vector, which means makes it's length equal to 1
    pub fn normalize(&self) -> Vector {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

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

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
   