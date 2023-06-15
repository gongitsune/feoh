pub mod aabb;
pub mod bvh;
pub mod hittable_list;
pub mod moving_sphere;
pub mod sphere;
pub mod xy_rect;

use crate::{hittable::aabb::AABB, material::Material, ray::Ray};
use glam::Vec3A;

pub struct HitRecord<'a> {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f32, f32)) -> Option<AABB>;
}
