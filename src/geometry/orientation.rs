use std::f32::consts::PI;

use crate::Vec3;

pub struct Orientation {
    rotation_deg: i16,
    roll_deg: i16,
    pitch_deg: i16
}

impl Orientation {
    pub fn new(mut rotation_deg: i16, mut roll_deg: i16, mut pitch_deg: i16) -> Self {
        rotation_deg = rotation_deg % 360;
        match rotation_deg {
            _r @ 0..=359 => (),
            r @ -359..=-1 => rotation_deg = 360 + r,
            _ => panic!("Value 'rotation_deg' incorrectly normalized, got {}", rotation_deg)
        } 

        roll_deg = roll_deg % 360;
        match roll_deg {
            _y @ 0..=359 => (),
            y @ -359..=-1 => roll_deg = 360 + y,
            _ => panic!("Value 'roll_deg' incorrectly normalized, got {}", roll_deg)
        }

        pitch_deg = pitch_deg % 360;
        match pitch_deg {
            _p @ 0..=359 => (),
            p @ -359..=-1 => pitch_deg = 360 + p,
            _ => panic!("Value 'pitch_deg' incorrectly normalized, got {}", pitch_deg)
        } 

        Orientation {
            rotation_deg,
            roll_deg,
            pitch_deg
        }
    }

    pub fn rotation_deg(&self) -> i16 {
        self.rotation_deg
    }
    pub fn rotation_rad(&self) -> f32 {
        self.rotation_deg as f32 * PI / 180f32
    }

    pub fn roll_deg(&self) -> i16 {
        self.roll_deg
    }
    pub fn roll_rad(&self) -> f32 {
        self.roll_deg as f32 * PI / 180f32
    }

    pub fn pitch_deg(&self) -> i16 {
        self.pitch_deg
    }
    pub fn pitch_rad(&self) -> f32 {
        self.pitch_deg as f32 * PI / 180f32
    }

    pub fn calc_unit_vectors(&self) -> (Vec3, Vec3, Vec3)  {
        let x_i: f32 = self.rotation_rad().cos() * self.pitch_rad().cos();
        let x_j: f32 = self.pitch_rad().sin();
        let x_k: f32 = self.rotation_rad().sin() * self.pitch_rad().cos();
        let x: Vec3 = Vec3 { x: x_i, y: x_j, z: x_k };

        let y_hat: Vec3 = Vec3 { x: 0f32, y: 1f32, z: 0f32 };
        let z: Vec3 = x.cross(&y_hat);
        let z_roll_apply: Vec3 = z.rotate_by(&x, self.roll_rad());

        let y: Vec3 = z.cross(&x);
        (x, y, z_roll_apply)
    }
}