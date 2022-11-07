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

    println!("Transformation: {}", transformation.to_string());

    let mut vector = Vector::new(1.0, 0.0, 0.0);

    vector = vector * transformation;

    println!("Vector: {}", vector.to_string());
}

#[cfg(test)]
mod tests {
    use crate::{vector::Vector, point::Point};
    

}