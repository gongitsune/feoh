use glam::Vec3A;

use crate::{material::Material, ray::Ray};

use super::{aabb::AABB, HitRecord, Hittable};

pub enum Plane {
    YZ,
    ZX,
    XY,
}

pub struct AARect<M: Material + Sync> {
    plane: Plane,
    a: (f32, f32),
    b: (f32, f32),
    k: f32,
    material: M,
}

impl<M: Material + Sync> AARect<M> {
    pub fn new(plane: Plane, a: (f32, f32), b: (f32, f32), k: f32, material: M) -> Self {
        AARect {
            plane,
            a,
            b,
            k,
            material,
        }
    }
}

impl<M: Material + Sync> Hittable for AARect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = match &self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
            Plane::XY => (2, 0, 1),
        };
        let t = (self.k - ray.origin[k_axis]) / ray.direction[k_axis];
        if t < t_min || t > t_max {
            None
        } else {
            let a = ray.origin[a_axis] + t * ray.direction[a_axis];
            let b = ray.origin[b_axis] + t * ray.direction[b_axis];
            if a < self.a.0 || a > self.a.1 || b < self.b.0 || b > self.b.1 {
                None
            } else {
                let u = (a - self.a.0) / (self.a.1 - self.a.0);
                let v = (b - self.b.0) / (self.b.1 - self.b.0);
                let point = ray.at(t);
                let mut normal = Vec3A::ZERO;
                normal[k_axis] = 1.0;
                Some(HitRecord {
                    t,
                    u,
                    v,
                    point,
                    normal,
                    material: &self.material,
                })
            }
        }
    }

    fn bounding_box(&self, _: (f32, f32)) -> Option<AABB> {
        let min = Vec3A::new(self.a.0, self.b.0, self.k - 0.0001);
        let max = Vec3A::new(self.a.1, self.b.1, self.k + 0.0001);
        Some(AABB { min, max })
    }
}