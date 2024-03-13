use crate::vec3::Vec3;

pub trait Sdf3d: Sync {
    fn get_distance(&self, pos: &Vec3) -> f64;
    fn get_colour(&self, intersection_point: Vec3) -> sdl2::pixels::Color;
}

impl dyn Sdf3d {
    // direction should be a unit vector
    pub fn sphere_trace(sdf: &impl Sdf3d, start_point: Vec3, direction: Vec3, collision_distance: f64) -> Option<(sdl2::pixels::Color, Vec3)> {
        let mut current_point = start_point;
        let mut last_step = sdf.get_distance(&current_point);
        let mut earlier_step = sdf.get_distance(&current_point);

        for _ in 0..=128 {
            let step = sdf.get_distance(&current_point);
            if step > last_step + 1.0 && last_step > earlier_step + 1.0 {
                return None;
            }
            earlier_step = last_step;
            last_step = step;

            if step < collision_distance {
                return Some((sdf.get_colour(current_point), current_point));
            }
            current_point = current_point + direction * step;
        }

        return None;
    }

    // tetrahedron technique found at https://iquilezles.org/articles/normalsSDF/
    pub fn estimate_normal(sdf: &impl Sdf3d, point: Vec3, offset_size: f64) -> Vec3 {
        let p0: Vec3 = Vec3::new(1.0, -1.0, -1.0);
        let p1: Vec3 = Vec3::new(-1.0, -1.0, 1.0);
        let p2: Vec3 = Vec3::new(-1.0, 1.0, -1.0);
        let p3: Vec3 = Vec3::new(1.0, 1.0, -1.0);

        return (p0 * sdf.get_distance(&(point + p0 * offset_size))
            + p1 * sdf.get_distance(&(point + p1 * offset_size))
            + p2 * sdf.get_distance(&(point + p2 * offset_size))
            + p3 * sdf.get_distance(&(point + p3 * offset_size)))
            .normalise();
    }
}
