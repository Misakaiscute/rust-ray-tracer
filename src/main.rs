mod scene;
mod surface;
mod geometry;
mod image;

pub use geometry::point3::Point3;
pub use scene::camera::Camera;
pub use geometry::vec3::Vec3;
pub use surface::material::Material;
pub use surface::color::Color;

use crate::{geometry::{GeometryHolder, objects::{Plane, Sphere}, orientation::Orientation}, image::ppm_writer::PPMWriter, surface::Surface};

fn main() -> std::io::Result<()> {
    let camera: Camera = Camera::new(
        1920,
        1080,
        Point3 { x: -3f32, y: 4f32, z: 0f32 },
        Orientation::new(0, 0, -5),
        90,
        5
    ).unwrap();

    let objects: Vec<Box<dyn GeometryHolder>> = vec![
        Box::new(Plane::new(
            Point3 { x: 0f32, y: 0f32, z: 0f32 },
            f32::INFINITY, 
            f32::INFINITY,
            Orientation::new(0, 0, 0),
            Surface {
                color: Color::new(220, 220, 220).unwrap(),
                mat: Material::new(0.3, 10f32).unwrap()
            }
        )),
        Box::new(Sphere {
            r: 3f32,
            origin: Point3 { x: 8f32, y: 3f32, z: 5f32 },
            surface: Surface {
                color: Color::new(255, 0, 0).unwrap(),
                mat: Material::new(0.8, 20f32).unwrap()
            }
        }),
        Box::new(Sphere {
            r: 4f32,
            origin: Point3 { x: 12f32, y: 4f32, z: -5f32 },
            surface: Surface {
                color: Color::new(34, 133, 161).unwrap(),
                mat: Material::new(0.3, 100f32).unwrap()
            }
        })
    ];

    let scene: scene::Scene = scene::Scene {
        light_source: scene::light::LightSource {
            cords: Point3 { x: -10f32, y: 20f32, z: 10f32 },
            color: Color::new(255, 255, 255).unwrap(),
            amb_light: 0.1
        },
        camera: camera,
        objects: objects,
        backdrop: Color::new(0, 0, 0).unwrap()
    };

    let mut result_matrix =vec![
        vec![Color::new(0, 0, 0).unwrap(); scene.camera.res_w as usize];
        scene.camera.res_h as usize
    ];
    scene.begin_trace(&mut result_matrix);

    PPMWriter::write(result_matrix)?;

    Ok(())
}