use glam::Vec3A;

use crate::{hittable::HitRecord, ray::Ray, texture::Texture, Rand};

use super::Material;

pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Rand) -> Option<(Ray, Vec3A)> {
        None
    }

    fn color_emitted(&self, u: f32, v: f32, p: &Vec3A) -> Vec3A {
        self.emit.value(u, v, p)
    }
}
