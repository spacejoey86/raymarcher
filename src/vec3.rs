pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 {
            x: x,
            y: y,
            z: z,
        };
    }

    pub fn length(self) -> f64 {
        const BASE: i32 = 2;
        let sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        return sum.sqrt();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_vec() {
        let my_vec3 = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(my_vec3.length(), 0.0);
    }

    #[test]
    fn pythag_quad() {
        let my_vec3 = Vec3::new(2.0, 3.0, 6.0);
        assert_eq!(my_vec3.length(), 7.0);
    }

    #[test]
    fn negative() {
        let my_vec3 = Vec3::new(-4.0, -4.0, -7.0);
        assert_eq!(my_vec3.length(), 9.0);
    }

    #[test]
    fn mixed_sign() {
        let my_vec3 = Vec3::new(1.0, 4.0, -8.0);
        assert_eq!(my_vec3.length(), 9.0);
    }
}