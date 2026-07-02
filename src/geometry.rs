use crate::{Point3, Vec3, surface::Surface};

pub trait GeometryHolder {
    fn normal_v(&self, point: &Point3) -> Vec3;
    fn intersect_d(&self, source: &Point3, d_v: &Vec3) -> Option<f32>;
    fn surface(&self) -> Surface;
}

pub mod point3;
pub mod vec3;
pub mod matrix3;
pub mod objects;
pub mod orientation;