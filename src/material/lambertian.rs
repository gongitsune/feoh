use super::Material;
use crate::{
    hittable::HitRecord,
    onb::Onb,
    ray::Ray,
    texture::{solid_color::SolidColor, Texture},
    vec::random_cosine_direction,
    Rand,
};
use glam::Vec3A;
use std::f32::consts::PI;

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
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rand) -> Option<(Ray, Vec3A, f32)> {
        let uvw = Onb::build_from_w(&hit.normal);
        let direction = uvw.local(&random_cosine_direction(rng));
        let scatterd = Ray::new(hit.point, direction.normalize(), ray.time);

        let pdf = uvw.w().dot(scatterd.direction) / PI;
        let albedo = self.albedo.value(hit.u, hit.v, &hit.point);
        Some((scatterd, albedo, pdf))
    }

    fn scattering_pdf(&self, _ray: &Ray, hit: &HitRecord, scatterd: &Ray) -> f32 {
        let cosine = hit.normal.dot(scatterd.direction.normalize());
        if cosine < 0.0 {
            0.
        } else {
            cosine / PI
        }
    }
}
