use std::cmp::max;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x: x, y: y, z: z };
    }

    pub fn splat(val: f64) -> Vec3 {
        return Vec3::new(val, val, val);
    }

    pub fn length(&self) -> f64 {
        let sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        return sum.sqrt();
    }

    pub fn abs(&self) -> Vec3 {
        return Vec3::new(self.x.abs(), self.y.abs(), self.z.abs());
    }

    pub fn clamp_lower(&self, val: f64) -> Vec3 {
        return Vec3::new(self.x.max(val), self.y.max(val), self.z.max(val));
    }

    pub fn max_component(&self) -> f64 {
        return self.x.max(self.y.max(self.z))
    }

    pub fn normalise(&self) -> Vec3 {
        return *self / self.length();
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn rotate(&self, pitch: f64, yaw: f64, roll: f64) -> Vec3 {
        let cosa = yaw.cos();
        let sina = yaw.sin();

        let cosb = pitch.cos();
        let sinb = pitch.sin();

        let cosc = roll.cos();
        let sinc = roll.sin();

        let axx = cosa*cosb;
        let axy = cosa*sinb*sinc - sina*cosc;
        let axz = cosa*sinb*cosc + sina*sinc;

        let ayx = sina*cosb;
        let ayy = sina*sinb*sinc + cosa*cosc;
        let ayz = sina*sinb*cosc - cosa*sinc;

        let azx = -sinb;
        let azy = cosb*sinc;
        let azz = cosb*cosc;

        let px = self.x;
        let py = self.y;
        let pz = self.z;


        return Vec3::new(axx*px + axy*py + axz*pz,
            ayx*px + ayy*py + ayz*pz,
            azx*px + azy*py + azz*pz);
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        return Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Vec3::new(-self.x, -self.y, -self.z);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // length
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

    // absolute value
    #[test]
    fn zero_length() {
        let my_vec3 = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(my_vec3.abs(), my_vec3);
    }

    #[test]
    fn mixed_sign_abs() {
        let my_vec3 = Vec3::new(1.0, -1.0, -2.0);
        assert_eq!(my_vec3.abs(), Vec3::new(1.0, 1.0, 2.0))
    }
}
