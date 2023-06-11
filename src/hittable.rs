pub mod hittable_list;
pub mod sphere;

use glam::Vec3A;

use crate::{material::Material, ray::Ray};

pub struct HitRecord<'a> {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
