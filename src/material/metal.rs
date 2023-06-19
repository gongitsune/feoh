use super::{random_in_unit_sphere, reflect, Material};
use crate::{hittable::HitRecord, ray::Ray, Rand};
use glam::Vec3A;

pub struct Metal {
    albedo: Vec3A,
    fuzzy: f32,
}

impl Metal {
    pub fn new(albedo: Vec3A, fuzzy: f32) -> Self {
        Self {
            albedo,
            fuzzy: fuzzy.min(1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rand) -> Option<(Ray, Vec3A, f32)> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        if reflected.dot(hit.normal) > 0. {
            let scatterd = Ray::new(
                hit.point,
                reflected + self.fuzzy * random_in_unit_sphere(rng),
                ray.time,
            );
            Some((scatterd, self.albedo, 0.))
        } else {
            None
        }
    }
}
