use super::{get_face_normal, HitRecord, Hittable};
use crate::{hittable::aabb::AABB, material::Material, ray::Ray};
use glam::Vec3A;
use std::f32::consts::PI;

pub struct Sphere<M: Material> {
    center: Vec3A,
    radius: f32,
    material: M,
}

fn get_sphere_uv(p: &Vec3A) -> (f32, f32) {
    let theta = -p.y.acos();
    let phi = -p.z.atan2(p.x) + PI;

    (phi / (2. * PI), theta / PI)
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3A, radius: f32, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material + Sync> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t_min < t && t < t_max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord {
                    point,
                    normal: get_face_normal(ray, normal),
                    t,
                    u,
                    v,
                    material: &self.material,
                });
            }
            let t = (-b + sqrt_discriminant) / a;
            if t_min < t && t < t_max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
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

    fn bounding_box(&self, _: (f32, f32)) -> Option<AABB> {
        let radius = Vec3A::splat(self.radius);
        Some(AABB::new(self.center - radius, self.center + radius))
    }
}
