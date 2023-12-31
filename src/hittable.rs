pub mod aabb;
pub mod boxtype;
pub mod bvh;
pub mod flip_face;
pub mod hittable_list;
pub mod moving_sphere;
pub mod rect;
pub mod rotate;
pub mod sphere;
pub mod translate;

use crate::{hittable::aabb::AABB, material::Material, ray::Ray, Rand};
use glam::Vec3A;

pub struct HitRecord<'a> {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f32, f32)) -> Option<AABB>;
    fn pdf_value(&self, _origin: Vec3A, _v: Vec3A) -> f32 {
        0.
    }
    fn random(&self, _origin: Vec3A, _rng: &mut Rand) -> Vec3A {
        Vec3A::new(1., 0., 0.)
    }
}

pub fn get_face_normal(ray: &Ray, outward_normal: Vec3A) -> (bool, Vec3A) {
    let front_face = ray.direction.dot(outward_normal) < 0.;
    (
        front_face,
        if front_face {
            outward_normal
        } else {
            -outward_normal
        },
    )
}
