use crate::sdf3d::Sdf3d;
use crate::vec3::Vec3;

pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        return Sphere {
            radius: radius,
        }
    }
}

impl Sdf3d for Sphere {
    fn get_distance(self, pos: Vec3) -> f64 {
        return pos.length() - self.radius;
    }
}
