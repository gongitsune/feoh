use std::marker::PhantomData;

use glam::Vec3A;
use rand::Rng;

use crate::{material::Material, ray::Ray};

use super::{HitRecord, Hittable};

pub struct Sphere<M: Material> {
    center: Vec3A,
    radius: f32,
    material: M,
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
                return Some(HitRecord {
                    point,
                    normal,
                    t,
                    material: &self.material,
                });
            }
            let t = (-b + sqrt_discriminant) / a;
            if t_min < t && t < t_max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord {
                    point,
                    normal,
                    t,
                    material: &self.material,
                });
            }
        }

        None
    }
}
