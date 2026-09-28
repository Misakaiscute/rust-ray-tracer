use std::f32::consts::PI;

use crate::{Point3, Vec3, geometry::orientation::Orientation};

pub struct Camera {
    ///Number of pixels horizontally
    pub width: u16,
    ///Number of pixels vertically
    pub height: u16,
    ///Distance to camera from origins
    d_to_cam: u16,
    ///Center point of the camera
    origin: Point3,
    ///The orientation of the camera compared to x hat
    orientation: Orientation,
    ///Vector perpendicular to the camera
    x_v: Vec3,
    ///Vector paralel to the width of the camera
    z_v: Vec3,
    ///Vector paralel to the height of the camera
    y_v: Vec3,
    ///Field of view in radians
    fov_rad: f32,
    ///Point of view, calculated from the width, direction, fov, and center of the camera
    pub pov: Point3,
    ///Left to right pixel shifting vector
    px_shift_vx: Vec3,
    ///Bottom to top pixel shifting vector
    px_shift_vy: Vec3,
    ///Bottom left pixel origin
    px_topleft_pos: Point3
}

impl Camera {
    pub fn new(width: u16, height: u16, origin: Point3, orientation: Orientation, fov_deg: u8, d_to_cam: u16) -> Result<Self, String> {
        let validation_result = Self::validate_params(fov_deg, d_to_cam);
        if let Some(x) = validation_result {
            return Err(x);
        }
        let fov_rad: f32 = fov_deg as f32 * PI / 180f32;

        let (x_v, y_v, z_v): (Vec3, Vec3, Vec3) = orientation.calc_unit_vectors();

        let pov: Point3 = Self::calc_pov(d_to_cam, &origin, &x_v);
        let viewport_g_x: f32 = Self::calc_viewport_g_x(d_to_cam, fov_rad);
        let viewport_g_y: f32 = Self::calc_viewport_g_y(d_to_cam, fov_rad, width, height);
        let px_shift_vx: Vec3 = Self::calc_px_shift_vx(&z_v, viewport_g_x, width);
        let px_shift_vy: Vec3 = Self::calc_px_shift_vy(&y_v, viewport_g_y, height);
        let px_topleft_pos: Point3 = Self::calc_topleft_px_cords(&origin, viewport_g_x, &z_v, viewport_g_y, &y_v);

        Ok(Camera {
            width,
            height,
            d_to_cam,
            origin,
            orientation,
            x_v,
            y_v,
            z_v,
            fov_rad,
            pov,
            px_shift_vx,
            px_shift_vy,
            px_topleft_pos
        })
    }

    pub fn set_dimensions(&mut self, width: u16, height: u16) {
        let viewport_g_x: f32 = Self::calc_viewport_g_x(self.d_to_cam, self.fov_rad);
        let viewport_g_y: f32 = Self::calc_viewport_g_y(self.d_to_cam, self.fov_rad, width, height);
        let px_shift_vx: Vec3 = Self::calc_px_shift_vx(&self.z_v, viewport_g_x, width);
        let px_shift_vy: Vec3 = Self::calc_px_shift_vy(&self.y_v, viewport_g_y, height);
        let px_topleft_pos: Point3 = Self::calc_topleft_px_cords(&self.origin, viewport_g_x, &self.z_v, viewport_g_y, &self.y_v);

        self.width = width;
        self.height = height;
        self.px_shift_vx = px_shift_vx;
        self.px_shift_vy = px_shift_vy;
    }

    pub fn px_to_ray(&self, px_idx_x: u16, px_idx_y: u16) -> Vec3 {
        let shift_vx: Vec3 = self.px_shift_vx.scale_by(f32::from(px_idx_x));
        let shift_vy: Vec3 = self.px_shift_vy.scale_by(f32::from(px_idx_y));
        let shift: Vec3 = shift_vx - shift_vy;
        let px: Point3 = self.px_topleft_pos.apply_vector(&shift);

        self.pov.derive_vector(&px).to_unit()
    }

    fn calc_pov(d_to_cam: u16, origin: &Point3, o_v: &Vec3) -> Point3 {
        origin.apply_vector(&o_v.scale_by(-f32::from(d_to_cam)))
    }

    fn calc_px_shift_vx(z_v: &Vec3, g_x: f32, width: u16) -> Vec3 {
        let px_shift_scale: f32 = (2.0 * g_x) / f32::from(width - 1);

        z_v.scale_by(px_shift_scale)
    }

    fn calc_px_shift_vy(y_v: &Vec3, g_y: f32, height: u16) -> Vec3 {
        let py_shift_scale: f32 = (2.0 * g_y) / f32::from(height - 1);

        y_v.scale_by(py_shift_scale)
    }

    fn calc_topleft_px_cords(origin: &Point3, g_x: f32, z_v: &Vec3, g_y: f32, y_v: &Vec3) -> Point3 {
        let v_z: Vec3 = z_v.scale_by(-g_x);
        let v_y: Vec3 = y_v.scale_by(g_y);
        let shift: Vec3 = v_z + v_y;

        origin.apply_vector(&shift)
    }

    fn calc_viewport_g_x(d_to_cam: u16, fov_rad: f32) -> f32 {
        f32::from(d_to_cam) * (f32::from(fov_rad) / 2f32).tan()
    }

    fn calc_viewport_g_y(d_to_cam: u16, fov_rad: f32, width: u16, height: u16) -> f32 {
        Self::calc_viewport_g_x(d_to_cam, fov_rad) * (f32::from(height - 1) / f32::from(width - 1))
    }

    fn validate_params(fov: u8, d_to_cam: u16) -> Option<String> {
        if !(1..=180).contains(&fov) {
            return Some(format!("Field of View must be between >= 1 and < 180, got {}", fov));
        } else if d_to_cam < 1 {
            return Some(format!("The distance fromt the camera must be at least 1 unit, got {}", d_to_cam));
        } 

        return None;
    }
}
