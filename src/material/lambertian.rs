use super::{random_in_unit_sphere, Material};
use crate::{hittable::HitRecord, ray::Ray, Rand};
use glam::Vec3A;

pub struct Lambertian {
    albedo: Vec3A,
}

impl Lambertian {
    pub fn new(albedo: Vec3A) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord, rng: &mut Rand) -> Option<(Ray, Vec3A)> {
        let target = hit.point + hit.normal + random_in_unit_sphere(rng);
        let scatterd = Ray::new(hit.point, target - hit.point);

        Some((scatterd, self.albedo))
    }
}
