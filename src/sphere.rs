use crate::sdf3d::Sdf3d;
use crate::vec3::Vec3;

pub struct Sphere {
    radius: f64,
    colour: sdl2::pixels::Color,
}

impl Sphere {
    pub fn new(radius: f64, colour: sdl2::pixels::Color) -> Sphere {
        return Sphere {
            radius: radius,
            colour: colour,
        }
    }
}

impl Sdf3d for Sphere {
    fn get_distance(&self, pos: &Vec3) -> f64 {
        return pos.length() - self.radius;
    }

    fn get_colour(&self, intersectionPoint: Vec3) -> sdl2::pixels::Color {
        return self.colour;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::pixels::Color;

    #[test]
    fn origin() {
        let my_sphere = Sphere::new(1.0, Color::RED);
        assert_eq!(my_sphere.get_distance(&Vec3::splat(0.0)), -1.0);
    }

    #[test]
    fn edge() {
        let my_sphere = Sphere::new(1.0, Color::RED);
        assert_eq!(my_sphere.get_distance(&Vec3::new(1.0, 0.0, 0.0)), 0.0);
    }

    #[test]
    fn past() {
        let my_sphere = Sphere::new(1.0, Color::RED);
        assert_eq!(my_sphere.get_distance(&Vec3::new(2.0, 0.0, 0.0)), 1.0);
    }
}
