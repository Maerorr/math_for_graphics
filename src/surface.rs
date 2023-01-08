use crate::point::Point;
use crate::vector::Vector;

// Surface is defined by a point and a normal vector
pub struct Surface {
    pub point: Point,
    pub normal: Vector,
}

impl Surface {
    pub fn new(point: Point, normal: Vector) -> Surface {
        Surface { point, normal }
    }

    // return the distance from the surface to a point
    pub fn distance(&self, point: &Point) -> f64 {
        let v = point.to_vector() - self.point.to_vector();
        v.dot(&self.normal)
    }

    // return the point on the surface closest to the given point
    pub fn closest_point(&self, point: &Point) -> Point {
        let v = point.to_vector() - self.point.to_vector();
        let d = v.dot(&self.normal);
        let v = self.normal * d;
        Point::from_vector(&(point.to_vector() - v))
    }
}