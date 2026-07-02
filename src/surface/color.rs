use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: u16, g: u16, b: u16) -> Result<Self, String> {
        let rgb_specs_valid: Option<String> = Color::validate_rgb_space(r, g, b);

        let r_n: f32 = f32::from(r) / 255f32;
        let g_n: f32 = f32::from(g) / 255f32;
        let b_n: f32 = f32::from(b) / 255f32;

        if rgb_specs_valid == None {
            return Ok(Color { r: r_n, g: g_n, b: b_n });
        } 

        return Err(rgb_specs_valid.unwrap());
    }

    pub fn scale_by(&self, s: f32) -> Self {
        let r: f32 = self.r * s;
        let g: f32 = self.g * s;
        let b: f32 = self.b * s;
        let mut result: Color = Color { r, g, b };
        result.normalize();

        result
    }

    pub fn take_least(&self, other: Self) -> Self {
        let mut r: f32 = self.r;
        let mut g: f32 = self.g;
        let mut b: f32 = self.b;

        if other.r < self.r {
            r = other.r;
        }
        if other.g < self.g {
            g = other.g;
        }
        if other.b < self.b {
            b = other.b;
        }
        Color { r, g, b }
    }

    pub fn as_rgb(&self) -> String {
        String::from(format!("{} {} {}", (self.r * 255f32) as u8, (self.g * 255f32) as u8, (self.b * 255f32) as u8))
    }

    fn normalize(&mut self) {
        if self.r > 1f32 {
            self.r = 1f32
        } else if self.r < -1f32 {
            self.r = -1f32
        }
    
        if self.g > 1f32 {
            self.g = 1f32
        } else if self.g < -1f32 {
            self.g = -1f32
        }

        if self.b > 1f32 {
            self.b = 1f32
        } else if self.b < -1f32 {
            self.b = -1f32
        }
    }

    fn validate_rgb_space(r: u16, g: u16, b: u16) -> Option<String>  {
        if !(0..=255).contains(&r) {
            return Some(format!("Red component must be >= 0 and < 256, got {}", r));
        } else if !(0..=255).contains(&g) {
            return Some(format!("Green component must be >= 0 and < 256, got {}", g));
        } else if !(0..=255).contains(&b) {
            return Some(format!("Blue component must be >= 0 and < 256, got {}", r));
        } 

        return None;
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let r: f32 = self.r + rhs.r;
        let g: f32 = self.g + rhs.g;
        let b: f32 = self.b + rhs.b;
        let mut result: Color = Color { r, g, b };
        result.normalize();

        result
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let r: f32 = self.r - rhs.r;
        let g: f32 = self.g - rhs.g;
        let b: f32 = self.b - rhs.b;
        let mut result: Color = Color { r, g, b };
        result.normalize();

        result
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let r: f32 = self.r * rhs.r;
        let g: f32 = self.g * rhs.g;
        let b: f32 = self.b * rhs.b;
        let mut result: Color = Color { r, g, b };
        result.normalize();

        result
    }
}