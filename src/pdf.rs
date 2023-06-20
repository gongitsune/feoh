use crate::{hittable::Hittable, onb::Onb, vec::random_cosine_direction, Rand};
use glam::Vec3A;
use rand::Rng;
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

pub struct MixturePdf {
    pub pdf: (Arc<dyn Pdf>, Arc<dyn Pdf>),
}

impl MixturePdf {
    pub fn new(pdf: (Arc<dyn Pdf>, Arc<dyn Pdf>)) -> Self {
        Self { pdf }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3A) -> f32 {
        0.5 * self.pdf.0.value(direction) + 0.5 * self.pdf.1.value(direction)
    }

    fn generate(&self, rng: &mut Rand) -> Vec3A {
        if rng.gen::<f32>() < 0.5 {
            self.pdf.0.generate(rng)
        } else {
            self.pdf.1.generate(rng)
        }
    }
}
