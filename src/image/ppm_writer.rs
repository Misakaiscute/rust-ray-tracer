use std::fs::File;
use std::io::Write;

use crate::Color;
use crate::image::ColorMatrix;

pub struct PPMWriter;

impl PPMWriter {
    pub fn write(color_matrix: ColorMatrix, name: &str) -> std::io::Result<()> {
        let mut file: File = File::create(format!("{name}.ppm")).unwrap();

        let y_size: usize = color_matrix.len();
        let x_size: usize = color_matrix[0].len();

        writeln!(file, "P3")?;
        writeln!(file, "{x_size} {y_size}")?;
        writeln!(file, "255")?;

        for y in 0..y_size {
            for x in (0..x_size).rev() {
                let pixel: Color = color_matrix[y][x];

                write!(file, "{} ", pixel.as_rgb())?;
            }
            write!(file, "\n")?;
        }

        Ok(())
    }
}
