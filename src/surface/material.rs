#[derive(Clone, Copy)]
pub struct Material {
    specularity: f32,
    sharpness: f32,
}

impl Material {
    pub fn new(specularity: f32, sharpness: f32) -> Result<Self, String> {
        let refl_specularitys_valid: Option<String> = Material::validate_refl_specularitys(specularity, sharpness);
        if let None = refl_specularitys_valid {
            return Ok(Material { specularity, sharpness });
        } 

        return Err(refl_specularitys_valid.unwrap());
    }

    pub fn specular_refl(&self) -> f32 {
        self.specularity
    }
    pub fn sharpness(&self) -> f32 {
        self.sharpness
    }

    fn validate_refl_specularitys(specularity: f32, sharpness: f32) -> Option<String> {
        if !(0f32..=1f32).contains(&specularity) {
            return Some(format!("specularityular reflection must be > 0 and <= 1, got {}", specularity));
        } else if !(0f32..=255f32).contains(&sharpness) {
            return Some(format!("Shine must be > 0 and < 256, got {}", sharpness));
        }

        return None;
    }
}