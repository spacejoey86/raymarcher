use crate::sdf3d::Sdf3d;
use crate::vec3::Vec3;

use std::cmp::{min, max};

pub struct Cuboid {
    half_size: Vec3,
    colours: [sdl2::pixels::Color; 6],
}

impl Cuboid {
    pub fn new(half_size: Vec3, colour: sdl2::pixels::Color) -> Cuboid {
        return Cuboid {
            half_size: half_size,
            colours: [colour; 6],
        }
    }

    pub fn coloured_cube(half_size: Vec3, colours: [sdl2::pixels::Color; 6]) -> Cuboid {
        return Cuboid {
            half_size: half_size,
            colours: colours,
        }
    }
}

impl Sdf3d for Cuboid {
    fn get_distance(&self, pos: &Vec3) -> f64 {
        let transformed_coord = pos.abs() - self.half_size;
        return transformed_coord.clamp_lower(0.0).length() + transformed_coord.max_component().min(0.0);
    }

    fn get_colour(&self, intersection_point: Vec3) -> sdl2::pixels::Color {
        if intersection_point.x >= self.half_size.x {
            return self.colours[0];
        } else if intersection_point.x <= -self.half_size.x {
            return self.colours[1];
        } else if intersection_point.y >= self.half_size.y {
            return self.colours[2];
        } else if intersection_point.y <= -self.half_size.y {
            return self.colours[3];
        } else if intersection_point.z >= self.half_size.z {
            return self.colours[4];
        } else if intersection_point.z <= -self.half_size.z {
            return self.colours[5];
        } else {
            // not on a side?
            return sdl2::pixels::Color::RGB(255, 0, 220);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::pixels::Color;

    #[test]
    fn origin_cube() {
        let my_cube = Cuboid::new(Vec3::new(5.0, 5.0, 5.0), Color::RED);
        assert_eq!(my_cube.get_distance(&Vec3::splat(0.0)), -5.0);
    }

    #[test]
    fn twice_past_cube() {
        let my_cube = Cuboid::new(Vec3::new(5.0, 5.0, 5.0), Color::RED);
        assert_eq!(my_cube.get_distance(&Vec3::new(0.0, 0.0, 10.0)), 5.0);
    }
}