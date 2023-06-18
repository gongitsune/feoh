use super::Material;
use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{solid_color::SolidColor, Texture},
    Rand,
};
use glam::Vec3A;

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    #[allow(dead_code)]
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}

impl From<Vec3A> for DiffuseLight<SolidColor> {
    fn from(value: Vec3A) -> Self {
        Self {
            emit: SolidColor::new(value),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Rand) -> Option<(Ray, Vec3A)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: &Vec3A) -> Vec3A {
        self.emit.value(u, v, p)
    }
}
