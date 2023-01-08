use float_cmp::{approx_eq, F64Margin};
use vector::*;
use point::*;
use mat4::*;
use crate::line::Line;
use crate::math::as_radians;
use crate::quaternion::Quaternion;

mod vector;
mod point;
mod mat4;
mod math;
mod quaternion;
mod surface;
mod line;

pub fn intersection_between_two_lines_and_angle_between_them() {
    println!("Excercise 1 and 2. Find an intersection point (if exists) between two lines and angle between them");
    let p1 = Vector::new(-2.0, 5.0, 0.0);
    let p2 = Vector::new(-2.0, 5.0, 0.0);
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

pub fn intersection_between_two_lines_but_using_structs() {
    println!("INTERSECTIONS BUT USING NEW STRUCTS");
    let p1 = Vector::new(-2.0, 5.0, 0.0);
    let p2 = Vector::new(-2.0, 5.0, 0.0);
    let v1 = Vector::new(3.0, 1.0, 5.0);
    let v2 = Vector::new(1.0, -5.0, 3.0);

    let line1 = Line::new(p1, v1);
    let line2 = Line::new(p2, v2);

    let intersection = line1.intersection(&line2);
    match intersection {
        Some(p) => println!("intersection: {}", p.to_string()),
        None => println!("no intersection"),
    }
}

pub fn intersection_between_line_and_surface_and_angle_between_them() {
    let p = Vector::new(-2.0, 2.0, -1.0);
    let v = Vector::new(3.0, -1.0, 2.0);

    let n = Vector::new(2.0, 3.0, 3.0);
    let q = Vector::new(4.0, 0.0, 0.0);

    let parallel_check = v.dot(&n);

    if approx_eq!(f64, parallel_check, 0.0, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
        println!("The line is parallel to the surface");
    } else {
        let t = ((n * -1.0).dot(&(p - q))) / n.dot(&v);
        let intersect_point = p + v * t;
        println!("\nExcercise 3 and 4. Find an intersection point (if exists) between a line and a surface and angle between them");
        println!("intersect_point: {}", intersect_point.to_string());
        let angle = v.angle_degrees(&n);
        println!("angle: {:.2} deg", angle);
        println!("because our 'angle' is between line and the surface normal it will be 90 - (angle between line and surface)");
    }
}

pub fn intersection_line_surface_but_new_structs() {
    let p = Vector::new(-2.0, 2.0, -1.0);
    let v = Vector::new(3.0, -1.0, 2.0);

    let n = Vector::new(2.0, 3.0, 3.0);
    let q = Vector::new(4.0, 0.0, 0.0);

    let line = Line::new(p, v);
    let surface = surface::Surface::new(q, n);

    let intersection = line.intersection_surface(&surface);
    match intersection {
        Some(p) => println!("intersection: {}", p.to_string()),
        None => println!("no intersection"),
    }
}

pub fn intersection_line_of_two_surfaces() {
    println!("\nExcercise 5 and 6. Find an intersection line (if exists) between two surfaces and angle between them");
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


pub fn intersection_of_two_segments() {
    println!("\nExcercise 7 and  8. Find an intersection point (if exists) between two segments");
    // segment 1
    let p1 = Vector::new(5.0, 5.0, 4.0);
    let q1 = Vector::new(10.0, 10.0, 6.0);

    // segment 2
    let p2 = Vector::new(5.0, 5.0, 5.0);
    let q2 = Vector::new(10.0, 10.0, 3.0);

    // vectors of these segments
    let v1 = Vector::from_points(&Point::from_vector(&p1), &Point::from_vector(&q1));
    let v2 = Vector::from_points(&Point::from_vector(&p2), &Point::from_vector(&q2));

    // now, lines that these segments are on have the following equations:
    // L1 = p1 + t1 * v1
    // L2 = p2 + t2 * v2
    // if t1 and t2 are between 0 and 1 the point lies on the segment

    // to find the intersection first we need to find the normal of the plane
    // that contains p1, p2 and q1
    let w = q1 - p1;
    let n = v1.cross(&w);

    // now we need to check if q2 is also on the plane
    // to find this we need a dot product of (q2 - p1) and n
    let dot = (q2 - p1).dot(&n);
    if !approx_eq!(f64, dot, 0.0, F64Margin { epsilon: f64::EPSILON, ulps: 4 }) {
        println!("the plane does not contain all the points");
    } else {
        // firstly we need to calculate normals to both line segments
        let n1 = v1.cross(&n);
        let n2 = v2.cross(&n);

        // then, test p1 and p2 agains the second line
        let t_p1 = (p1 - q1).dot(&n2);
        let t_p2 = (p2 - q1).dot(&n2);
        if t_p2 * t_p1 >= 0.0 {
            // similarly we check q1 and q2
            let t_q1 = (q1 - q1).dot(&n1);
            let t_q2 = (q2 - q1).dot(&n1);

            if t_q2 * t_q1 >= 0.0 {
                // finally we can check where the intersection is
                let cross_squared = v1.cross(&v2).length() * v1.cross(&v2).length();

                let t1 = (p2 - p1).cross(&v2).dot(&v1.clone().cross(&v2)) / cross_squared;
                let t2 = (p2 - p1).cross(&v1).dot(&v1.clone().cross(&v2)) / cross_squared;

                let p3 = p1 + v1 * t1;
                let p4 = p2 + v2 * t2;

                // they should be equal if there is an intersection
                println!("p3: {}", p3.to_string());
                println!("p4: {}", p4.to_string());
                println!("this is our intersection point");
            } else {
                println!("tested q1 and q2 are not on the opposite sides");
            }
        } else {
            println!("tested p1 and p2 are not on the opposite sides");
        }
    }
}

pub fn first_intersection_point_of_line_and_sphere() {
    println!("\nExcercise 9. Find one intersection point of a line and a sphere");
    let c = Vector::new(0.0, 0.0, 0.0);
    let r_pow2 = 26_f64;
    // changing the point of viev aka swapping p and q changes which intersection we get
    let q = Vector::new(3.0, -1.0, -2.0);
    let p = Vector::new(5.0, 3.0, -4.0);
    let mut v = q - p;
    // this normalization is needed
    v.normalize();
    let w = c - p;
    //println!("v: {}", v.to_string());
    //println!("w: {}", w.to_string());
    let l = w.dot(&v);

    // both a and d are SQUARED values
    let a = w.length().powi(2) - l.powi(2);
    let d = r_pow2 - a;

    if a > d {
        println!("there is no intersection");
    } else {
        let intersection = p + (v * (l - d.sqrt()));
        println!("intersection point: {}", intersection.to_string());
    }
}

fn main() {
}