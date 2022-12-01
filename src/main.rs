use float_cmp::{approx_eq, F64Margin};
use vector::*;
use point::*;
use mat4::*;
use crate::math::as_radians;
use crate::quaternion::Quaternion;

mod vector;
mod point;
mod mat4;
mod math;
mod quaternion;

pub fn intersection_between_two_lines_and_angle_between_them() {
    println!("Excercise 1. Find an intersection point (if exists) between two lines and angle between them");
    let p1 = Vector::new(-2.0, 5.0, 0.0);
    let p2 = Vector::new(-2.0, 4.0, 0.0);
    let v1 = Vector::new(3.0, 1.0, 5.0);
    let v2 = Vector::new(1.0, -5.0, 3.0);

    let cross_squared = v1.cross(&v2).length() * v1.cross(&v2).length();

    let t1 = (p2 - p1).cross(&v2).dot(&v1.clone().cross(&v2)) / cross_squared;
    let t2 = (p2 - p1).cross(&v1).dot(&v1.clone().cross(&v2)) / cross_squared;

    let p3 = p1 + v1 * t1;
    let p4 = p2 + v2 * t2;

    // they should be equal if there is an intersection
    println!("p3: {}", p3.to_string());
    println!("p4: {}", p4.to_string());

    let angle = v1.angle_degrees(&v2);
    println!("angle: {:.2} deg", angle);
}

pub fn intersection_between_line_and_surface_and_angle_between_them() {
    let p = Vector::new(-2.0, 2.0, -1.0);
    let v = Vector::new(3.0, -1.0, 2.0);

    let n = Vector::new(2.0, 3.0, 3.0);
    // here, equation 2x+3y+3z-8=0 is solved for (x,y,z) = (1,1,1) or (4,0,0)
    let q = Vector::new(4.0, 0.0, 0.0);

    let parallel_check = v.dot(&n);

    if approx_eq!(f64, parallel_check, 0.0, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
        println!("The line is parallel to the surface");
    } else {
        let t = ((n * -1.0).dot(&(p - q))) / n.dot(&v);
        let intersect_point = p + v * t;
        println!("\nExcercise 2. Find an intersection point (if exists) between a line and a surface and angle between them");
        println!("intersect_point: {}", intersect_point.to_string());
        let angle = v.angle_degrees(&n);
        println!("angle: {:.2} deg", angle);
    }
}

pub fn intersection_line_of_two_surfaces() {
    println!("\nExcercise 3. Find an intersection line (if exists) between two surfaces");
    // 2x - y + z - 8 = 0
    // 4x + 3y +z + 14 = 0
    let n1 = Vector::new(2.0, -1.0, 1.0);
    let n2 = Vector::new(4.0, 3.0, 1.0);
    let q1 = Vector::new(0.0, 0.0, 8.0);
    let q2 = Vector::new(0.0, 0.0, -14.0);

    let n_cross = n1.cross(&n2);
    if approx_eq!(f64, n_cross.length(), 0.0, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
        println!("The surfaces are parallel");
    } else {
        // find a point on the intersecting line
        let mut out_p = Vector::new(0.0, 0.0, 0.0);
        if !approx_eq!(f64, n_cross.z, 0.0, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
            // this is some basic 2 variable linear equation solving using gaussian elimination
            let (mut a1,mut  b1,mut  c1) = (n1.x, n1.y, n1.x* q1.x + n1.y * q1.y + n1.z * q1.z);
            let (mut a2,mut  b2,mut  c2) = (n2.x, n2.y, n2.x* q2.x + n2.y * q2.y + n2.z * q2.z);
            let mut temp = a2 / a1;
            a1 *= temp;
            b1 *= temp;
            c1 *= temp;
            a2 -= a1;
            b2 -= b1;
            c2 -= c1;
            temp = b1 / b2;
            a2 *= temp;
            b2 *= temp;
            c2 *= temp;
            b1 -= b2;
            c1 -= c2;
            out_p.x = c1 / a1;
            out_p.y = c2 / b2;
        }
        println!("intersecting line: {} + t * {}", out_p.to_string(), n_cross.to_string());
    }

    println!("angle between the surfaces: {:.2} deg", n1.angle_degrees(&n2).to_string());
}



fn main() {
    intersection_between_two_lines_and_angle_between_them();
    intersection_between_line_and_surface_and_angle_between_them();
    intersection_line_of_two_surfaces();
    // todo - angle between two segments, and intersection between line and a sphere
}