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

fn main() {
    println!("\n-----------------------------START------------------------------\n");
    let mut vect = Vector::new(-1.0, -1.0, -1.0);
    let mut vect_mat = Vector::new(-1.0, -1.0, -1.0);
    println!("We have a vector: {}", vect.to_string());
    let mut q = Quaternion::identity();
    println!("Create a unit quaternion: {}", q.to_string());
    q.rotate(as_radians(270.0), Vector::new(1.0, 0.0, 0.0));
    println!("Rotate the quaternion by 270 deg on x axis: {}", q.to_string());
    q.rotate_vec(&mut vect);
    println!("Rotate the vector by this quaternion: {}", vect.to_string());

    let mat = q.to_mat4();
    vect_mat = vect_mat * mat;
    println!("Rotating the same vector with quaternion as a matrix: {}", vect_mat.to_string());

    println!("\n----------------------------------------------------------------\n");
    let mut q1 = Quaternion::identity();
    let mut q2 = Quaternion::identity();
    q1.rotate(as_radians(42.0), Vector::new(1.0, 0.0, 0.0));
    q2.rotate(as_radians(133.0), Vector::new(0.0, 0.0, 1.0));
    println!("Create two quaternions: {} and {}", q1.to_string(), q2.to_string());

    let q3 = q1 * q2;
    let q4 = q2 * q1;
    println!("q1 * q2: {}", q3.to_string());
    println!("q2 * q1: {}", q4.to_string());
    println!("q1 * q2 != q2 * q1");
    println!("\n-----------------------------END--------------------------------\n");
}