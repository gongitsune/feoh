use super::{random_in_unit_sphere, reflect, Material};
use crate::{ray::Ray, Rand};
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
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        hit: &crate::hittable::HitRecord,
        rng: &mut Rand,
    ) -> Option<(crate::ray::Ray, Vec3A)> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        if reflected.dot(hit.normal) > 0. {
            let scatterd = Ray::new(
                hit.point,
                reflected + self.fuzzy * random_in_unit_sphere(rng),
            );
            Some((scatterd, self.albedo))
        } else {
            None
        }
    }
}
