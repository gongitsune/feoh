use super::{aabb::AABB, HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

pub struct FlipFace<H: Hittable> {
    pub hittable: Arc<H>,
}

impl<H: Hittable> FlipFace<H> {
    pub fn new(hittable: Arc<H>) -> Self {
        Self { hittable }
    }
}

impl<H: Hittable> Hittable for FlipFace<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut hit) = self.hittable.hit(ray, t_min, t_max) {
            hit.front_face = !hit.front_face;
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, time: (f32, f32)) -> Option<AABB> {
        self.hittable.bounding_box(time)
    }
}
