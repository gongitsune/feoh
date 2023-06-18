use glam::Vec3A;

use crate::ray::Ray;

use super::{get_face_normal, Hittable};

pub struct Translate<H: Hittable> {
    pub hittable: H,
    pub offset: Vec3A,
}

impl<H: Hittable> Translate<H> {
    #[allow(dead_code)]
    pub fn new(hittable: H, offset: Vec3A) -> Self {
        Self { hittable, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if let Some(mut hit) = self.hittable.hit(&moved_ray, t_min, t_max) {
            hit.point += self.offset;
            hit.normal = get_face_normal(&moved_ray, hit.normal);

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, time: (f32, f32)) -> Option<super::aabb::AABB> {
        if let Some(mut bbox) = self.hittable.bounding_box(time) {
            bbox.min += self.offset;
            bbox.max += self.offset;

            Some(bbox)
        } else {
            None
        }
    }
}
