use crate::{Point3, Surface, Vec3, geometry::{GeometryHolder, orientation::Orientation}};

pub struct Sphere {
    pub r: f32,
    pub origin: Point3,
    pub surface: Surface
}

impl super::GeometryHolder for Sphere {
    fn normal_v(&self, point: &Point3) -> Vec3 {
        self.origin.derive_vector(&point).to_unit()
    }
    fn intersect_d(&self, source: &Point3, ray_v: &Vec3) -> Option<f32> {
        let v: Vec3 = self.origin.derive_vector(source);
        let b: f32 = v.scale_by(2f32).dot(ray_v);
        let c: f32 = v.dot(&v) - self.r.powi(2);
        let discriminant: f32 = b.powi(2) - 4f32 * c;

        if discriminant < 0f32 {
            return None;
        } else if discriminant == 0f32 {
            let solution: f32 = -b / 2f32;
            if solution <= 0f32 {
                return None;
            }
            return Some(solution);
        }

        let solutions: Vec<f32> = vec![
            (-b - discriminant.sqrt()) / 2f32,
            (-b + discriminant.sqrt()) / 2f32
        ];
        let intersect_d: Option<f32> = solutions
            .into_iter()
            .filter(|x| *x > 0f32)
            .min_by(|a, b| a.total_cmp(b));

        if let Some(x) = intersect_d {
            return Some(x);
        }
        return None;
    }
    fn surface(&self) -> Surface {
        self.surface
    }
}

pub struct Plane {
    origin: Point3,
    x_dim: f32,
    z_dim: f32,
    orientation: Orientation,
    x_v: Vec3,
    y_v: Vec3,
    z_v: Vec3,
    pub surface: Surface
}

impl GeometryHolder for Plane {
    fn normal_v(&self, _point: &Point3) -> Vec3 {
        self.y_v
    }
    fn intersect_d(&self, source: &Point3, ray_v: &Vec3) -> Option<f32> {
        let d_to_plane: f32 = self.y_v.dot(&source.derive_vector(&self.origin)) / self.y_v.dot(ray_v);

        if d_to_plane < 0f32 {
            return None
        }
        //Allow for infinite plane
        if self.x_dim == f32::INFINITY && self.z_dim == f32::INFINITY {
            return Some(d_to_plane);
        }

        let pt_hit: Point3 = source.apply_vector(&ray_v.scale_by(d_to_plane));
        let origin_to_pt: Vec3 = pt_hit.derive_vector(&self.origin);

        let x_len: f32 = origin_to_pt.dot(&self.x_v);
        let z_len: f32 = origin_to_pt.dot(&self.z_v);

        if (0f32 <= x_len && x_len <= (self.x_dim / 2f32).abs()) && (0f32 <= z_len && z_len <= (self.z_dim / 2f32).abs()) {
            return Some(d_to_plane);
        }
        return None;
    }
    fn surface(&self) -> Surface {
        self.surface
    }
}

impl Plane {
    pub fn new(origin: Point3, x_dim: f32, z_dim: f32, orientation: Orientation, surface: Surface) -> Self {
        let (x_v, y_v, z_v): (Vec3, Vec3, Vec3) = orientation.calc_unit_vectors();

        Plane {
            origin,
            x_dim,
            z_dim,
            orientation,
            x_v,
            y_v,
            z_v,
            surface
        }
    }
}