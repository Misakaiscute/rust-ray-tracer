mod scene;
mod surface;
mod geometry;
mod image;
mod anti_aliasing;

pub use geometry::point3::Point3;
pub use scene::camera::Camera;
pub use geometry::vec3::Vec3;
pub use surface::material::Material;
pub use surface::color::Color;

use crate::{anti_aliasing::AntiAliasing, geometry::{GeometryHolder, objects::{Plane, Sphere}, orientation::Orientation}, image::{ColorMatrix, ppm_writer::PPMWriter}, scene::{Scene, light::LightSource}, surface::Surface};

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
                color: Color::new(255, 255, 255).unwrap(),
                mat: Material::new(0.3, 10f32).unwrap()
            }
        )),
        Box::new(Sphere {
            r: 3f32,
            origin: Point3 { x: 8f32, y: 3f32, z: 5f32 },
            surface: Surface {
                color: Color::new(255, 0, 0).unwrap(),
                mat: Material::new(0.05, 20f32).unwrap()
            }
        }),
        Box::new(Sphere {
            r: 4f32,
            origin: Point3 { x: 12f32, y: 4f32, z: -10f32 },
            surface: Surface {
                color: Color::new(34, 133, 161).unwrap(),
                mat: Material::new(0.3, 100f32).unwrap()
            }
        })
    ];

    let mut scene: Scene = Scene::new(
        LightSource {
            cords: Point3 { x: -10f32, y: 20f32, z: 10f32 },
            color: Color::WHITE,
            amb_light: 0.1
        },
        camera,
        objects,
        Color::new(98, 158, 227).unwrap(),
        AntiAliasing::SSAA(4)
    ).unwrap();

    let image: ColorMatrix = scene.render();
    PPMWriter::write(image, "anti_aliasing_4x")?;

    Ok(())
}
