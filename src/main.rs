use vector::*;
use point::*;
use mat4::*;
use crate::math::as_radians;

mod vector;
mod point;
mod mat4;
mod math;

fn main() {
    let mut transformation = Mat4::identity();
    transformation.rotate(as_radians(90.0), Vector::new(0.0, 1.0, 0.0));

    let mut vector = Vector::new(1.0, 0.0, 0.0);

    println!("We take out vector: {}", vector.to_string());
    println!("We rotate it by 90 degrees around the y axis with the following matrix:");
    println!("{}", transformation.to_string());

    vector = vector * transformation;

    println!("Our vector rotated by 90deg on Y-axis: {}\n", vector.to_string());

    println!("Now, we will show that matrix multiplication is not commutative.");

    let mut m1 = Mat4::identity();
    let mut m2 = Mat4::identity();

    m1.translate(Vector::new(1.0, 2.0, 3.0));
    m2.translate(Vector::new(3.0, 2.0, 1.0));
    m1.rotate(as_radians(90.0), Vector::new(0.0, 1.0, 0.0));
    m2.rotate(as_radians(123.0), Vector::new(0.0, 1.0, 0.0));
    m1.rotate(as_radians(45.0), Vector::new(0.0, 0.0, 1.0));
    m2.rotate(as_radians(45.0), Vector::new(1.0, 0.0, 0.0));

    println!("We have two matrices: \n{}and \n{}", m1.to_string(), m2.to_string());
    println!("We multiply them in the following order: m1 * m2");
    let m3 = m1 * m2;
    println!("We get the following matrix: \n{}", m3.to_string());
    println!("Now, we multiply them in the opposite order: m2 * m1");
    let m4 = m2 * m1;
    println!("We get the following matrix: \n{}", m4.to_string());
    println!("As we can see, the order of multiplication matters.");

    println!("\n\nMULTIPLICATION ORDER TEST");

    let mut m1 = Mat4::identity();
    let mut m2 = Mat4::identity();
    let mut vec1 = Vector::new(1.0, 1.0, 1.0);
    let mut vec2 = Vector::new(1.0, 1.0, 1.0);

    m1.translate(Vector::new(1.0, 2.0, 3.0));
    m2.scale(Vector::new(2.0, 5.0, 10.0));

    println!("We have a vector {}", vec1.to_string());
    println!("We have two matrices: \n{}and \n{}", m1.to_string(), m2.to_string());

    vec1 = vec1 * m1 * m2;
    vec2 = vec2 * m2 * m1;

    println!("v1 * m1 * m2 = {}", vec1.to_string());
    println!("v1 * m2 * m1 = {}", vec2.to_string());

    println!("CONCLUSION: RUST MULTIPLIES FROM LEFT TO RIGHT");
    println!("as if you typed '(vec * m1) * m2' ");
}