use super::{random_in_unit_sphere, Material};
use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{solid_color::SolidColor, Texture},
    Rand,
};
use glam::Vec3A;

pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Self { albedo: texture }
    }
}

impl From<Vec3A> for Lambertian<SolidColor> {
    fn from(value: Vec3A) -> Self {
        Self {
            albedo: SolidColor::new(value),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rand) -> Option<(Ray, Vec3A)> {
        let target = hit.point + hit.normal + random_in_unit_sphere(rng);
        let scatterd = Ray::new(hit.point, target - hit.point, ray.time);

        Some((scatterd, self.albedo.value(hit.u, hit.v, &hit.point)))
    }
}
