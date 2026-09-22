use crate::{Camera, Color, Point3, Vec3, geometry::GeometryHolder};

pub struct Scene {
    pub light_source: light::LightSource,
    pub camera: Camera,
    pub objects: Vec<Box<dyn GeometryHolder>>,
    pub backdrop: Color
}

impl Scene {
    const MAX_BOUNCES: u8 = 5;
    const EPSILON: f32 = 0.001;
    pub fn begin_trace(&self, result_matrix: &mut Vec<Vec<Color>>) -> () {
        for y in (0..(self.camera.res_h as usize)).rev() {
            for x in 0..self.camera.res_w as usize {
                let ray_v: Vec3 = self.camera.px_to_ray(x as u16, y as u16);
                let px_color: Color = self.trace_ray(
                    &self.camera.pov,
                    ray_v,
                    self.light_source.color,
                    Color::new(0, 0, 0).unwrap(),
                    None, None
                );
                result_matrix[y][x] = px_color;
            }
        }
    }

    fn trace_ray(
        &self,
        ray_source: &Point3,
        ray_v: Vec3,
        light_color: Color,
        mut deduced_color: Color,
        retained_spec: Option<f32>,
        bounces_left: Option<u8>
    ) -> Color {
        //Determine the object index we hit
        let obj_hit: Option<(usize, f32)> = self.calc_obj_hit(ray_source, &ray_v);
        //Base cases
        if obj_hit == None {
            return deduced_color + self.backdrop.scale_by(retained_spec.unwrap_or(1f32));
        } else if bounces_left.unwrap_or(Self::MAX_BOUNCES) == 0 {
            return deduced_color;
        }
        //Calculate next bounce ray_v and source
        let (obj_idx, dist_to_obj) = obj_hit.unwrap();
        let point_hit: Point3 = ray_source.apply_vector(&ray_v.scale_by(dist_to_obj));
        let n_v: Vec3 = self.objects[obj_idx].normal_v(&point_hit).to_unit();
        let l_v: Vec3 = point_hit.derive_vector(&self.light_source.cords).to_unit();
        let incident: f32 = n_v.dot(&ray_v);
        let r_v: Vec3 = ray_v - n_v.scale_by(2f32 * incident);
        
        //Update color on pixel
        let current_color: Color = self.shade(&point_hit, obj_idx, &ray_v, &n_v, &l_v, retained_spec.unwrap_or(1f32), light_color);
        deduced_color = deduced_color + current_color;

        //Recoursion
        return self.trace_ray(
            &point_hit.apply_vector(&r_v.scale_by(Self::EPSILON)),
            r_v,
            current_color.take_least(light_color),
            deduced_color,
            Some(self.objects[obj_idx].surface().mat.specular_refl() * retained_spec.unwrap_or(1f32)),
            Some(bounces_left.unwrap_or(Self::MAX_BOUNCES) - 1),
        )
    }

    fn calc_obj_hit(&self, ray_source: &Point3, ray_v: &Vec3) -> Option<(usize, f32)> {
        let mut obj_intersect_dist: Vec<Option<f32>> = vec![];
        for shape in self.objects.iter() {
            let hit_dist: Option<f32> = shape.intersect_d(ray_source, ray_v);
            obj_intersect_dist.push(hit_dist);
        }
        let next_hit_dist: Option<f32> = obj_intersect_dist
            .iter()
            .flatten()
            .copied()
            .min_by(|a, b| a.total_cmp(b));

        if next_hit_dist == None {
            return None;
        }

        let obj_idx: Option<usize> = obj_intersect_dist
            .iter()
            .position(|x| *x != None && *x == next_hit_dist);
        
        return Some((obj_idx.unwrap(), next_hit_dist.unwrap()));
    }

    fn calc_shadow_attenuation(&self, for_point: &Point3, l_v: &Vec3, for_obj_idx: usize) -> Color {
        let epsilon_adjusted_point: Point3 = for_point.apply_vector(&l_v.scale_by(Self::EPSILON));
        let result: Option<(usize, f32)> = self.calc_obj_hit(&epsilon_adjusted_point, l_v);
        if result == None {
            return self.light_source.color;
        } 

        let (hit_obj_idx, dist_to_obj) = result.unwrap();
        if hit_obj_idx == for_obj_idx {
            return self.light_source.color;
        }

        let to_hit_v: Vec3 = l_v.scale_by(dist_to_obj);
        let to_l_v: Vec3 = for_point.derive_vector(&self.light_source.cords);
        if to_hit_v.magnitude() > to_l_v.magnitude() {
            return self.light_source.color;
        } else {
            return Color::new(0, 0, 0).unwrap();
        }
    }

    fn shade(&self, point_hit: &Point3, obj_idx: usize, ray_v: &Vec3, n_v: &Vec3, l_v: &Vec3, last_bounce_spec: f32, light_color: Color) -> Color {
        let shadow_attenuation: Color = self.calc_shadow_attenuation(point_hit, l_v, obj_idx);
        let ambient: Color = self.objects[obj_idx].surface().calc_ambient(self.light_source.amb_light);
        let diffuse: Color = self.objects[obj_idx].surface().calc_diffuse(n_v, l_v, light_color);
        let specular: Color = self.objects[obj_idx].surface().calc_specular(ray_v, n_v, l_v, light_color);
        
        (ambient + diffuse + specular).scale_by(last_bounce_spec) * shadow_attenuation
    }
}

pub mod camera;
pub mod light;
