use crate::{Color, Point3};

pub struct LightSource {
    pub cords: Point3,
    pub color: Color,
    pub amb_light: f32
}