use super::{aabb::AABB, HitRecord, Hittable};
use crate::material::Material;
use glam::Vec3A;

pub struct XYRect<M: Material + Sync> {
    pub material: M,
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub k: f32,
}

impl<M: Material + Sync> Hittable for XYRect<M> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t < t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return None;
        }

        let normal = Vec3A::new(0., 0., 1.);
        Some(HitRecord {
            point: ray.at(t),
            normal,
            t,
            u: (x - self.x.0) / (self.x.1 - self.x.0),
            v: (y - self.y.0) / (self.y.1 - self.y.0),
            material: &self.material,
        })
    }

    fn bounding_box(&self, _: (f32, f32)) -> Option<super::aabb::AABB> {
        Some(AABB::new(
            Vec3A::new(self.x.0, self.y.0, self.k - 0.0001),
            Vec3A::new(self.x.1, self.y.1, self.k + 0.0001),
        ))
    }
}
