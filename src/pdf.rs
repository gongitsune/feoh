use crate::{hittable::Hittable, onb::Onb, vec::random_cosine_direction, Rand};
use glam::Vec3A;
use std::{f32::consts::PI, sync::Arc};

pub trait Pdf {
    fn value(&self, direction: Vec3A) -> f32;
    fn generate(&self, rng: &mut Rand) -> Vec3A;
}

pub struct CosinePdf {
    pub uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3A) -> Self {
        Self {
            uvw: Onb::build_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3A) -> f32 {
        let cosine = direction.normalize().dot(self.uvw.w());
        if cosine <= 0. {
            0.
        } else {
            cosine / PI
        }
    }

    fn generate(&self, rng: &mut Rand) -> Vec3A {
        self.uvw.local(&random_cosine_direction(rng))
    }
}

pub struct HittablePdf<H: Hittable> {
    pub origin: Vec3A,
    pub hittable: Arc<H>,
}

impl<H: Hittable> HittablePdf<H> {
    pub fn new(origin: Vec3A, hittable: Arc<H>) -> Self {
        Self { origin, hittable }
    }
}

impl<H: Hittable> Pdf for HittablePdf<H> {
    fn value(&self, direction: Vec3A) -> f32 {
        self.hittable.pdf_value(self.origin, direction)
    }

    fn generate(&self, rng: &mut Rand) -> Vec3A {
        self.hittable.random(self.origin, rng)
    }
}
