use crate::{Color, image::ColorMatrix};

pub enum AntiAliasing {
    None,
    SSAA(u16),
}

impl AntiAliasing {
    pub fn apply(&self, render: ColorMatrix) -> ColorMatrix {
        let result: ColorMatrix = match self {
            Self::SSAA(sampling) => Self::ssaa(*sampling, &render),
            _ => render
        };
        result
    }

    fn ssaa(sampling: u16, render: &ColorMatrix) -> ColorMatrix {
        let height: usize = render.len() / sampling as usize;
        let width: usize = (*render[0]).len() / sampling as usize;
        let mut result: ColorMatrix = vec![
            vec![Color::BLACK; width]; height
        ];
        let px_count: u16 = sampling * sampling;

        for y in 0..height {
            for x in 0..width {
                let mut r_chn: i16 = 0;
                let mut g_chn: i16 = 0;
                let mut b_chn: i16 = 0;

                let xo_lower: usize = x * sampling as usize;
                let xo_upper: usize = (x * sampling as usize) + sampling as usize;
                let yo_lower: usize = y * sampling as usize;
                let yo_upper: usize = (y * sampling as usize) + sampling as usize;

                for yo in yo_lower..yo_upper {
                    for xo in xo_lower..xo_upper {
                        r_chn += render[yo][xo].red();
                        g_chn += render[yo][xo].green();
                        b_chn += render[yo][xo].blue();
                    }
                }
                let r: i16 = r_chn / px_count as i16;
                let g: i16 = g_chn / px_count as i16;
                let b: i16 = b_chn / px_count as i16;
                let px_avg: Color = Color::new(r, g, b).unwrap();

                result[y][x] = px_avg;
            }
        }
        result
    }

    pub fn render_size(&self, origin_w: u16, origin_h: u16) -> (u16, u16) {
        return match self {
            Self::SSAA(sampling) => (origin_w * sampling, origin_h * sampling),
            _ => (origin_w, origin_h)
        }
    }
}
