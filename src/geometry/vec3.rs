use crate::geometry::matrix3::Matrix3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn to_unit(&self) -> Self {
        let magnitude: f32 = self.magnitude();
        Vec3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude
        }
    }
    pub fn transform_by(&self, m: Matrix3) -> Self {
        let x: f32 = self.x * m.x_i + self.y * m.y_i + self.z * m.z_i;
        let y: f32 = self.x * m.x_j + self.y * m.y_j + self.z * m.z_j;
        let z: f32 = self.x * m.x_k + self.y * m.y_k + self.z * m.z_k;
        Vec3 { x, y, z }
    }
    pub fn scale_by(&self, s: f32) -> Self {
        let x: f32 = self.x * s;
        let y: f32 = self.y * s;
        let z: f32 = self.z * s;
        Vec3 { x, y, z }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        let x: f32 = self.y * rhs.z - rhs.y * self.z;
        let y: f32 = self.z * rhs.x - rhs.z * self.x;
        let z: f32 = self.x * rhs.y - rhs.x * self.y;
        Vec3 { x, y, z }
    }
    pub fn rotate_by(&self, axis: &Vec3, by_rad: f32) -> Self {
        self.scale_by(by_rad.cos()) + axis.cross(self).scale_by(by_rad.sin()) + axis.scale_by(axis.dot(self)).scale_by(1f32 - by_rad.cos())
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let x: f32 = self.x + rhs.x;
        let y: f32 = self.y + rhs.y;
        let z: f32 = self.z + rhs.z;
        Vec3 { x, y, z }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let x: f32 = self.x - rhs.x;
        let y: f32 = self.y - rhs.y;
        let z: f32 = self.z - rhs.z;
        Vec3 { x, y, z }
    }
}