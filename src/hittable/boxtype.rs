use std::sync::Arc;

use crate::{material::Material, ray::Ray};

use super::{
    aabb::AABB,
    hittable_list::HittableList,
    rect::{AARect, Plane},
    HitRecord, Hittable,
};
use glam::Vec3A;

pub struct BoxType {
    pub min: Vec3A,
    pub max: Vec3A,
    pub sides: HittableList,
}

impl BoxType {
    pub fn new<M: Material + 'static>(point: (Vec3A, Vec3A), material: Arc<M>) -> Self {
        let mut sides = HittableList::default();
        sides.push(AARect::new(
            Plane::XY,
            (point.0.x, point.1.x),
            (point.0.y, point.1.y),
            point.1.z,
            material.clone(),
        ));
        sides.push(AARect::new(
            Plane::XY,
            (point.0.x, point.1.x),
            (point.0.y, point.1.y),
            point.0.z,
            material.clone(),
        ));

        sides.push(AARect::new(
            Plane::XZ,
            (point.0.x, point.1.x),
            (point.0.z, point.1.z),
            point.1.y,
            material.clone(),
        ));
        sides.push(AARect::new(
            Plane::XZ,
            (point.0.x, point.1.x),
            (point.0.z, point.1.z),
            point.0.y,
            material.clone(),
        ));

        sides.push(AARect::new(
            Plane::YZ,
            (point.0.y, point.1.y),
            (point.0.z, point.1.z),
            point.1.x,
            material.clone(),
        ));
        sides.push(AARect::new(
            Plane::YZ,
            (point.0.y, point.1.y),
            (point.0.z, point.1.z),
            point.0.x,
            material.clone(),
        ));

        Self {
            min: point.0,
            max: point.1,
            sides,
        }
    }
}

impl Hittable for BoxType {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _: (f32, f32)) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
