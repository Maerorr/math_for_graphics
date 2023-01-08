use float_cmp::{approx_eq, F64Margin};
use crate::point::Point;
use crate::surface::Surface;
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

    pub fn angle_degrees(&self, other: &Line) -> f64 {
        let angle = self.direction.angle_degrees(&other.direction);
        angle
    }

    pub fn angle_radians(&self, other: &Line) -> f64 {
        let angle = self.direction.angle_radians(&other.direction);
        angle
    }

    // returns the expression p + tv
    pub fn point_on_line(&self, t: &f64) -> Vector {
        self.point + self.direction * *t
    }

    // Returns the point of intersection if they intersect. Otherwise returns None.
    pub fn intersection_surface(&self, surface: &Surface) -> Option<Vector> {
        let parallel_check = self.direction.dot(&surface.normal);
        if approx_eq!(f64, parallel_check, 0.0, F64Margin::default()) {
            None
        } else {
            let t = ((surface.normal * -1.0).dot(&(self.point - surface.point)))
                / (surface.normal.dot(&self.direction));
            let intersection = self.point_on_line(&t);
            Some(intersection)
        }
    }
}

