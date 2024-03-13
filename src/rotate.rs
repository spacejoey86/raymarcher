use crate::sdf3d::Sdf3d;

pub struct RotatedSdf<'a> {
    pitch: f64,
    yaw: f64,
    roll: f64,
    inner: &'a dyn Sdf3d,
}

impl<'a> RotatedSdf<'a> {
    pub fn new(pitch: f64, yaw: f64, roll: f64, inner: &dyn Sdf3d) -> RotatedSdf {
        return RotatedSdf {
            pitch: pitch,
            yaw: yaw,
            roll: roll,
            inner: inner,
        }
    }
}

impl<'a> Sdf3d for RotatedSdf<'a> {
    fn get_distance(&self, pos: &crate::vec3::Vec3) -> f64 {
        let rotated_coord = pos.rotate(self.pitch, self.yaw, self.roll);

        return self.inner.get_distance(&rotated_coord);
    }

    fn get_colour(&self, intersection_point: crate::vec3::Vec3) -> sdl2::pixels::Color {
        let rotated_coord = intersection_point.rotate(self.pitch, self.yaw, self.roll);

        return self.inner.get_colour(rotated_coord);
    }
}