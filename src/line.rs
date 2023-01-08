use float_cmp::{approx_eq, F64Margin};
use crate::point::Point;
use crate::vector::Vector;

pub struct Line {
    pub point: Vector,
    pub direction: Vector,
}

impl Line {
    pub fn new(point: Vector, direction: Vector) -> Line {
        Line { point, direction }
    }

    // Returns the point of intersection if they intersect. Otherwise returns None.
    pub fn intersection(&self, other: &Line) -> Option<Vector> {
        let cross_squared = self.direction.cross(&other.direction).length_squared();
        let t1 = (other.point - self.point)
            .cross(&other.direction)
            .dot(&self.direction.clone().cross(&other.direction))
            / cross_squared;
        let t2 = (other.point - self.point)
            .cross(&self.direction)
            .dot(&self.direction.clone().cross(&other.direction))
            / cross_squared;

        let p1 = self.point + self.direction * t1;
        let p2 = other.point + other.direction * t2;

        if p1 == p2 {
            // this returns one of the values since they are the same, or very, very close to each other.
            Some(p1)
        } else {
            None
        }
    }
}

