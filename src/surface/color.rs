use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
pub struct Color {
    r: i16,
    g: i16,
    b: i16,
}

impl Color {
    pub const BLACK: Self = Self {r: 0, g: 0, b: 0};
    pub const WHITE: Self = Self {r: 255, g: 255, b: 255};

    pub const COLOR_SPACE_MAX: i16 = 255;
    pub const COLOR_SPACE_MIN: i16 = 0;

    pub fn new(r: i16, g: i16, b: i16) -> Result<Self, String> {
        let rgb_specs_valid: Option<String> = Color::validate_rgb_space(r, g, b);

        if rgb_specs_valid == None {
            return Ok(Color { r: r, g: g, b: b });
        } 

        return Err(rgb_specs_valid.unwrap());
    }

    pub fn red(&self) -> i16 {
        self.r
    }
    pub fn green(&self) -> i16 {
        self.g
    }
    pub fn blue(&self) -> i16 {
        self.b
    }

    pub fn scale_by(&self, s: f32) -> Self {
        let r: f32 = f32::from(self.r) * s;
        let g: f32 = f32::from(self.g) * s;
        let b: f32 = f32::from(self.b) * s;
        let mut result: Color = Color {
            r: r.round() as i16,
            g: g.round() as i16,
            b: b.round() as i16
        };
        result.normalize();

        result
    }

    pub fn take_least(&self, other: Self) -> Self {
        let mut r: i16 = self.r;
        let mut g: i16 = self.g;
        let mut b: i16 = self.b;

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
        String::from(format!("{} {} {}", self.r, self.g, self.b))
    }

    fn normalize(&mut self) {
        if self.r > Self::COLOR_SPACE_MAX {
            self.r = Self::COLOR_SPACE_MAX
        } else if self.r <= Self::COLOR_SPACE_MIN {
            self.r = Self::COLOR_SPACE_MIN 
        }

        if self.g > Self::COLOR_SPACE_MAX {
            self.g = Self::COLOR_SPACE_MAX
        } else if self.g <= Self::COLOR_SPACE_MIN {
            self.g = Self::COLOR_SPACE_MIN 
        }

        if self.b > Self::COLOR_SPACE_MAX {
            self.b = Self::COLOR_SPACE_MAX
        } else if self.b <= Self::COLOR_SPACE_MIN {
            self.b = Self::COLOR_SPACE_MIN 
        }
    }

    fn validate_rgb_space(r: i16, g: i16, b: i16) -> Option<String>  {
        if !(0..=Self::COLOR_SPACE_MAX).contains(&r) {
            return Some(format!("Red component must be >= 0 and < 256, got {}", r));
        } else if !(0..=Self::COLOR_SPACE_MAX).contains(&g) {
            return Some(format!("Green component must be >= 0 and < 256, got {}", g));
        } else if !(0..=Self::COLOR_SPACE_MAX).contains(&b) {
            return Some(format!("Blue component must be >= 0 and < 256, got {}", r));
        } 

        return None;
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let r: i16 = self.r + rhs.r;
        let g: i16 = self.g + rhs.g;
        let b: i16 = self.b + rhs.b;
        let mut result: Color = Color { r, g, b };
        result.normalize();

        result
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let r: i16 = self.r - rhs.r;
        let g: i16 = self.g - rhs.g;
        let b: i16 = self.b - rhs.b;
        let mut result: Color = Color { r, g, b };
        result.normalize();

        result
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut r: i32 = (self.r as i32 * rhs.r as i32) / Self::COLOR_SPACE_MAX as i32;
        if r > i16::MAX as i32 {
            r = i16::MAX as i32
        }

        let mut g: i32 = (self.g as i32 * rhs.g as i32) / Self::COLOR_SPACE_MAX as i32;
        if g > i16::MAX as i32 {
            g = i16::MAX as i32
        }

        let mut b: i32 = (self.b as i32 * rhs.b as i32) / Self::COLOR_SPACE_MAX as i32;
        if b > i16::MAX as i32 {
            b = i16::MAX as i32
        }
        let mut result: Color = Color {
            r: r as i16,
            g: g as i16,
            b: b as i16
        };
        result.normalize();

        result
    }
}
