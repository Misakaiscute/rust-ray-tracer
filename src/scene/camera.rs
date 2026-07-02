use std::f32::consts::PI;

use crate::{Point3, Vec3, geometry::orientation::Orientation};

pub struct Camera {
    ///Number of pixels horizontally
    pub res_w: u16,
    ///Number of pixels vertically
    pub res_h: u16,
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
    px_loc_br: Point3
}

impl Camera {
    pub fn new(res_w: u16, res_h: u16, origin: Point3, orientation: Orientation, fov_deg: u8, d_to_camera: u16) -> Result<Self, String> {
        let validation_result = Camera::validate_params(fov_deg, d_to_camera);
        if let Some(x) = validation_result {
            return Err(x);
        }
        let fov_rad: f32 = fov_deg as f32 * PI / 180f32;

        let (x_v, y_v, z_v): (Vec3, Vec3, Vec3) = orientation.calc_unit_vectors();

        let pov: Point3 = Camera::calc_pov(d_to_camera, &origin, &x_v);

        let viewport_g_x: f32 = Camera::calc_viewport_g_x(d_to_camera, fov_rad);
        let viewport_g_y: f32 = Camera::calc_viewport_g_y(d_to_camera, fov_rad, res_w, res_h);
        let px_shift_vx: Vec3 = Camera::calc_px_shift_vx(&z_v, viewport_g_x, res_w);
        let px_shift_vy: Vec3 = Camera::calc_px_shift_vy(&y_v, viewport_g_y, res_h);
        let px_loc_br: Point3 = Camera::calc_br_px_cords(&origin, viewport_g_x, &z_v, viewport_g_y, &y_v);

        Ok(Camera {
            res_w,
            res_h,
            origin,
            orientation,
            x_v,
            y_v,
            z_v,
            fov_rad,
            pov,
            px_shift_vx,
            px_shift_vy,
            px_loc_br
        })
    }

    pub fn px_to_ray(&self, px_idx_x: u16, px_idx_y: u16) -> Vec3 {
        let shift_vx: Vec3 = self.px_shift_vx.scale_by(f32::from(px_idx_x));
        let shift_vy: Vec3 = self.px_shift_vy.scale_by(f32::from(px_idx_y));
        let shift: Vec3 = shift_vx + shift_vy;
        let px: Point3 = self.px_loc_br.apply_vector(&shift);

        self.pov.derive_vector(&px).to_unit()
    }

    fn calc_pov(d_to_camera: u16, origin: &Point3, o_v: &Vec3) -> Point3 {
        origin.apply_vector(&o_v.scale_by(-f32::from(d_to_camera)))
    }

    fn calc_px_shift_vx(z_v: &Vec3, g_x: f32, res_w: u16) -> Vec3 {
        let px_shift_scale: f32 = (2.0 * g_x) / f32::from(res_w - 1);

        z_v.scale_by(px_shift_scale)
    }

    fn calc_px_shift_vy(y_v: &Vec3, g_y: f32, res_h: u16) -> Vec3 {
        let py_shift_scale: f32 = (2.0 * g_y) / f32::from(res_h - 1);

        y_v.scale_by(py_shift_scale)
    }

    fn calc_br_px_cords(origin: &Point3, g_x: f32, z_v: &Vec3, g_y: f32, y_v: &Vec3) -> Point3 {
        let v_z: Vec3 = z_v.scale_by(-g_x);
        let v_y: Vec3 = y_v.scale_by(-g_y);
        let shift: Vec3 = v_z + v_y;

        origin.apply_vector(&shift)
    }

    fn calc_viewport_g_x(d_to_camera: u16, fov_rad: f32) -> f32 {
        f32::from(d_to_camera) * (f32::from(fov_rad) / 2f32).tan()
    }

    fn calc_viewport_g_y(d_to_camera: u16, fov_rad: f32, res_w: u16, res_h: u16) -> f32 {
        Camera::calc_viewport_g_x(d_to_camera, fov_rad) * (f32::from(res_h - 1) / f32::from(res_w - 1))
    }

    fn validate_params(fov: u8, d_to_camera: u16) -> Option<String> {
        if !(1..=180).contains(&fov) {
            return Some(format!("Field of View must be between >= 1 and < 180, got {}", fov));
        } else if d_to_camera < 1 {
            return Some(format!("The distance fromt the camera must be at least 1 unit, got {}", d_to_camera));
        } 

        return None;
    }
}