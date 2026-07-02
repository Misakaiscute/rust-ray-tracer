use crate::Vec3;

#[derive(Debug)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point3 {
    pub fn apply_vector(&self, v: &Vec3) -> Self {
        let x: f32 = self.x + v.x;
        let y: f32 = self.y + v.y;
        let z: f32 = self.z + v.z;
        Point3 { x, y, z }
    }
    pub fn derive_vector(&self, from: &Self) -> Vec3 {
        let x: f32 = from.x - self.x;
        let y: f32 = from.y - self.y;
        let z: f32 = from.z - self.z;
        Vec3 { x, y, z }
    }
}