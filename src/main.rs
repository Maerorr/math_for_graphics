use vector::*;
use point::*;
use mat4::*;

mod vector;
mod point;
mod mat4;
mod math;

fn main() {
    let mut m = Mat4::identity();
    let mut m2 = Mat4::new();
    m2.m[3][1] = 2.0;

    println!("{}", m.to_string());

    m.multiply(&m2);

    println!("{}", m.to_string());
}

#[cfg(test)]
mod tests {
    use crate::{vector::Vector, point::Point};
    

}