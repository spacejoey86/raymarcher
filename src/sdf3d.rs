use sdl2::pixels::Color;

use crate::vec3::Vec3;

#[derive(Clone)]
pub enum Sdf3d {
    Sphere{radius: f64, colour: Color},
    Cuboid{half_size: Vec3, colours: [Color; 6]},
    RotatedSdf{pitch: f64, yaw: f64, roll: f64, inner: Box<Sdf3d>},
}

impl Sdf3d {

    // direction should be a unit vector
    pub fn sphere_trace(sdf: &Sdf3d, start_point: Vec3, direction: Vec3, collision_distance: f64) -> Option<(sdl2::pixels::Color, Vec3)> {
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
    pub fn estimate_normal(sdf: &Sdf3d, point: Vec3, offset_size: f64) -> Vec3 {
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

    fn get_distance(&self, pos: &Vec3) -> f64 {
        match self {
            Sdf3d::Sphere { radius, .. } => {
                pos.length() - radius
            },
            Sdf3d::Cuboid { half_size, .. } => {
                let transformed_coord = pos.abs() - *half_size;
                transformed_coord.clamp_lower(0.0).length() + transformed_coord.max_component().min(0.0)
            },
            Sdf3d::RotatedSdf { pitch, yaw, roll, inner } => {
                let rotated_coord = pos.rotate(*pitch, *yaw, *roll);
                return inner.get_distance(&rotated_coord);
            },
        }
    }
    fn get_colour(&self, intersection_point: Vec3) -> sdl2::pixels::Color {
        match self {
            Sdf3d::Sphere { colour, .. } => {
                return *colour;
            },
            Sdf3d::Cuboid { half_size, colours } => {
                if intersection_point.x >= half_size.x {
                    return colours[0];
                } else if intersection_point.x <= -half_size.x {
                    return colours[1];
                } else if intersection_point.y >= half_size.y {
                    return colours[2];
                } else if intersection_point.y <= -half_size.y {
                    return colours[3];
                } else if intersection_point.z >= half_size.z {
                    return colours[4];
                } else if intersection_point.z <= -half_size.z {
                    return colours[5];
                } else {
                    // not on a side?
                    return sdl2::pixels::Color::RGB(255, 0, 220);
                }
            },
            Sdf3d::RotatedSdf { pitch, yaw, roll, inner } => {
                let rotated_coord = intersection_point.rotate(*pitch, *yaw, *roll);
                return inner.get_colour(rotated_coord);
            },
        }
    }
}
