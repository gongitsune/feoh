use super::{HitRecord, Hittable};
use crate::hittable::aabb::AABB;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Box::new(hittable))
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time: (f32, f32)) -> Option<AABB> {
        let mut output_box = None;
        for obj in self.objects.iter() {
            if let Some(bounding_box) = obj.bounding_box(time) {
                output_box = Some(match output_box {
                    Some(output_box) => AABB::surrounding_box(&output_box, &bounding_box),
                    None => bounding_box,
                });
            } else {
                return None;
            }
        }

        output_box
    }
}
