use crate::vec3::Vec3;

pub trait Sdf3d {
    fn get_distance(self, pos: Vec3) -> f64;
}