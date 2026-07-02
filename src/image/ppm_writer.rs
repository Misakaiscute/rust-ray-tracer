use std::fs::File;
use std::io::Write;

use crate::Color;

pub struct PPMWriter;

impl PPMWriter {
    pub fn write(color_matrix: Vec<Vec<Color>>) -> std::io::Result<()> {
        let mut file: File = File::create("result.ppm").unwrap();

        let y_size: usize = color_matrix.iter().count();
        let x_size: usize = color_matrix[0].iter().count();

        writeln!(file, "P3")?;
        writeln!(file, "{x_size} {y_size}")?;
        writeln!(file, "255")?;

        for y in (0..y_size).rev() {
            for x in (0..x_size).rev() {
                let pixel: Color = color_matrix[y][x];

                write!(file, "{} ", pixel.as_rgb())?;
            }
            write!(file, "\n")?;
        }

        Ok(())
    }
}