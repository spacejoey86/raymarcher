use crate::{direction::Direction, vec3::Vec3};

pub trait Sdf3d {
    fn get_distance(&self, pos: &Vec3) -> f64;
    fn get_colour(&self, intersection_point: Vec3) -> sdl2::pixels::Color;
}

impl dyn Sdf3d {
    // direction should be a unit vector
    pub fn sphere_trace(sdf: &impl Sdf3d, start_point: Vec3, direction: Vec3, collision_distance: f64) -> Option<(sdl2::pixels::Color, Vec3)> {
        let mut current_point = start_point;

        for _ in 0..=16 {
            let step = sdf.get_distance(&current_point);
            if step < collision_distance {
                return Some((sdf.get_colour(current_point), current_point));
            }
            current_point = current_point + direction * step;
        }

        return None;
    }

    // tetrahedron technique found at https://iquilezles.org/articles/normalsSDF/
    pub fn estimate_normal(sdf: &impl Sdf3d, point: Vec3, offset_size: f64) -> Vec3 {
        let P0: Vec3 = Vec3::new(1.0, -1.0, -1.0);
        let P1: Vec3 = Vec3::new(-1.0, -1.0, 1.0);
        let P2: Vec3 = Vec3::new(-1.0, 1.0, -1.0);
        let P3: Vec3 = Vec3::new(1.0, 1.0, -1.0);

        return (P0 * sdf.get_distance(&(point + P0 * offset_size))
            + P1 * sdf.get_distance(&(point + P1 * offset_size))
            + P2 * sdf.get_distance(&(point + P2 * offset_size))
            + P3 * sdf.get_distance(&(point + P3 * offset_size)))
            .normalise();
    }
}
