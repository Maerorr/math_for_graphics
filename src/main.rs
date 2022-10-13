use vector::*;
use point::*;

mod vector;
mod point;

fn main() {
    // Zaimplementuj klasę wektor wraz ze wszystkimi działaniami.
    //Sprawdź czy prawidłowo działa przemienność dodawania za pomocą odpowiedniego zdefiniowania przykładowych wektorów i ich sumy w funkcji main().

    //Znajdź kąt pomiędzy wektorem [0,3,0] a [5,5,0]

    println!("\n1) angle between [0,3,0] and [5,5,0]\n");

    let vec030: Vector = Vector::new(0.0, 3.0, 0.0);
    let vec550: Vector = Vector::new(5.0, 5.0, 0.0);

    println!("\tangle between {} and {} = {:.4} rad \n", vec030.to_string(), vec550.to_string(), vec030.angle(&vec550));

    //Znajdź wektor prostopadły do wektorów [4,5,1] i [4,1,3]

    println!("\n2) vector perpendicular to [4,5,1] and [4,1,3]\n");

    let vec451: Vector = Vector::new(4.0, 5.0, 1.0);
    let vec413: Vector = Vector::new(4.0, 1.0, 3.0);

    let mut perpendicular = vec451.cross(&vec413);
    println!("\tvector perpendicular to {} and {} = {}\n", vec451.to_string(), vec413.to_string(), perpendicular.to_string());

    //Znormalizuj powstały wektor

    println!("\n3) Normalize the prviously acquired vector\n");

    perpendicular.normalize();

    println!("\tNormalized vector = {}\n\n", perpendicular.to_string());


}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{vector::Vector, point::Point};
    
    #[test]
    fn sub_add_test() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        let v4 = v2 + v1;
        assert_eq!(v3, v4);
        let v5 = v3 - v2;
        assert_eq!(v1, v5);
    }

    #[test]
    fn mul_test() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        let v3 = v2 * 0.5;
        assert_eq!(v1, v3);
    }

    #[test]
    fn from_points_test() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(4.0, 5.0, 6.0);
        let v1 = Vector::from_points(&p1, &p2);
        let v2 = Vector::new(3.0, 3.0, 3.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn dot_test() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn cross_test() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = v1.cross(&v2);
        let v4 = Vector::new(-3.0, 6.0, -3.0);
        assert_eq!(v3, v4);
    }

    #[test]
    fn angle_test() {
        let v1 = Vector::new(0.0, 3.0, 0.0);
        let v2 = Vector::new(5.0, 5.0, 0.0);
        let mut angle = v1.angle(&v2);
        // simple trick to round decimal places
        // 0.123499 * 10000.0 = 1234.99 -> 1234 -> 1234.0 / 10000.0 = 0.1234
        angle = (angle*10000.0).round() / 10000.0;
        assert_eq!(angle, 0.7854);
    }

    #[test]
    fn normalize_test() {
        let vector = Vector::new(15.0, 12.0, -15.0);
        let mut normalized = vector.normalize();
        // because of float precision issues, we need to round the values.
        let vec_string = format!("{:.5}", normalized.length());
        assert_eq!(vec_string, "1.00000");
    }

    #[test]
    fn length_test() {
        let vector = Vector::new(1.0, 2.0, -3.0);
        let vec_string = format!("{:.4}", vector.length());
        assert_eq!(vec_string, "3.7417");
    }
}