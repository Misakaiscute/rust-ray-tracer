use crate::Vec3;

#[derive(Clone, Copy)]
pub struct Surface {
    pub color: color::Color,
    pub mat: material::Material
}

impl Surface {
    pub fn calc_diffuse(&self, n_v: &Vec3, l_v: &Vec3, l_i: color::Color) -> color::Color {
        let mut dot: f32 = n_v.dot(l_v);
        if dot < 0f32 {
            dot = 0f32;
        }
        self.color * l_i.scale_by(dot)
    }
    pub fn calc_specular(&self, ray_v: &Vec3, n_v: &Vec3, l_v: &Vec3, l_i: color::Color) -> color::Color {
        let h_v: Vec3 = ray_v.scale_by(-1f32) + *l_v;
        let mut dot: f32 = n_v.dot(&h_v.to_unit());
        if dot < 0f32 { 
            dot = 0f32;
        }
        l_i.scale_by(self.mat.specular_refl()).scale_by(dot.powf(self.mat.sharpness()))
    }
    pub fn calc_ambient(&self, l_amb: f32) -> color::Color {
        self.color.scale_by(l_amb)
    }
}

pub mod color;
pub mod material;