use super::{HitRecord, Hittable};
use crate::hittable::aabb::AABB;
use crate::material::Material;
use glam::Vec3A;
use std::f32::consts::PI;

pub struct MovingSphere<M: Material> {
    pub center: (Vec3A, Vec3A),
    pub time: (f32, f32),
    pub radius: f32,
    pub material: M,
}

fn get_sphere_uv(p: &Vec3A) -> (f32, f32) {
    let theta = -p.y.acos();
    let phi = -p.z.atan2(p.x) + PI;

    (phi / (2. * PI), theta / PI)
}

impl<M: Material + Sync> MovingSphere<M> {
    #[allow(dead_code)]
    pub fn new(center: (Vec3A, Vec3A), time: (f32, f32), radius: f32, material: M) -> Self {
        Self {
            center,
            time,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3A {
        self.center.0
            + ((time - self.time.0) / (self.time.1 - self.time.0)) * (self.center.1 - self.center.0)
    }
}

impl<M: Material + Sync> Hittable for MovingSphere<M> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;

            if t_min < t && t < t_max {
                let point = ray.at(t);
                let normal = (point - center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord {
                    point,
                    normal,
                    t,
                    u,
                    v,
                    material: &self.material,
                });
            }

            let t = (-b + sqrt_discriminant) / a;
            if t_min < t && t < t_max {
                let point = ray.at(t);
                let normal = (point - center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord {
                    point,
                    normal,
                    t,
                    u,
                    v,
                    material: &self.material,
                });
            }
        }

        None
    }

    fn bounding_box(&self, time: (f32, f32)) -> Option<AABB> {
        let radius = Vec3A::splat(self.radius);
        let box1 = AABB::new(self.center(time.0) - radius, self.center(time.0) + radius);
        let box2 = AABB::new(self.center(time.1) - radius, self.center(time.1) + radius);
        Some(AABB::surrounding_box(&box1, &box2))
    }
}
