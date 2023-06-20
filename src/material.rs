pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

use glam::Vec3A;
use rand::distributions::Uniform;

use crate::{hittable::HitRecord, ray::Ray, vec::random_vec, Rand};

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rand) -> Option<(Ray, Vec3A, f32)>;
    fn scattering_pdf(&self, _ray: &Ray, _hit: &HitRecord, _scatterd: &Ray) -> f32 {
        0.
    }
    fn emitted(&self, _ray: &Ray, _hit: &HitRecord) -> Vec3A {
        Vec3A::ZERO
    }
}

fn random_in_unit_sphere(rng: &mut Rand) -> Vec3A {
    const UNIT: Vec3A = Vec3A::splat(1.);
    let uniform = Uniform::new(0., 1.);
    loop {
        let p = 2. * random_vec(rng, uniform) - UNIT;
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: Vec3A, n: Vec3A) -> Vec3A {
    v - 2. * v.dot(n) * n
}

fn refract(v: Vec3A, n: Vec3A, ni_over_nt: f32) -> Option<Vec3A> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    let cos = 1. - cosine;
    let cos = cos * cos * cos * cos * cos;
    r0 + (1. - r0) * cos
}
