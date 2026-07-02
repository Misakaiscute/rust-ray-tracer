use crate::{Camera, Color, Point3, Vec3, geometry::GeometryHolder};

pub struct Scene {
    pub light_source: light::LightSource,
    pub camera: Camera,
    pub objects: Vec<Box<dyn GeometryHolder>>,
    pub backdrop: Color
}

impl Scene {
    pub fn begin_trace(&self, result_matrix: &mut Vec<Vec<Color>>) -> () {
        for y in (0..(self.camera.res_h as usize)).rev() {
            for x in 0..self.camera.res_w as usize {
                let ray_v: Vec3 = self.camera.px_to_ray(x as u16, y as u16);
                let px_color: Color = self.trace_ray(
                    Color::new(0, 0, 0).unwrap(),
                    self.light_source.color,
                    1f32,
                    &self.camera.pov,
                    ray_v,
                    5,
                    5
                );
                result_matrix[y][x] = px_color;
            }
        }
        /*let ray_v: Vec3 = self.camera.px_to_ray(776, 951);
        let px_color: Color = self.trace_ray(
            Color::new(0, 0, 0).unwrap(),
            self.light_source.color,
            1f32,
            &self.camera.pov,
            ray_v,
            5
        );
        result_matrix[776][951] = px_color;*/
    }

    fn trace_ray(
        &self,
        mut deduced_color: Color,
        light_color: Color,
        retained_spec: f32,
        ray_source: &Point3,
        ray_v: Vec3,
        max_bounces: u8,
        bounces_left: u8
    ) -> Color {
        let obj_hit: Option<(usize, f32)> = self.calc_obj_hit(ray_source, &ray_v);
        if obj_hit == None || bounces_left == 0 {
            if bounces_left == max_bounces {
                return self.backdrop;
            } else {
                return deduced_color;
            }
        }

        let (obj_idx, dist_to_obj) = obj_hit.unwrap();
        let point_hit: Point3 = ray_source.apply_vector(&ray_v.scale_by(dist_to_obj));
        let n_v: Vec3 = self.objects[obj_idx].normal_v(&point_hit).to_unit();
        let l_v: Vec3 = point_hit.derive_vector(&self.light_source.cords).to_unit();

        let current_color: Color = self.shade(&point_hit, obj_idx, &ray_v, &n_v, &l_v, retained_spec, light_color);
        deduced_color = deduced_color + current_color;

        let incident: f32 = n_v.dot(&ray_v);
        let r_v: Vec3 = ray_v - n_v.scale_by(2f32 * incident);
        
        return self.trace_ray(
            deduced_color,
            current_color.take_least(light_color),
            self.objects[obj_idx].surface().mat.specular_refl() * retained_spec,
            &point_hit.apply_vector(&r_v.scale_by(0.001)),
            r_v,
            max_bounces,
            bounces_left - 1,
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
        let epsilon_adjusted_point: Point3 = for_point.apply_vector(&l_v.scale_by(0.001));
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