use std::f32::consts::PI;

use glam::Vec3A;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::Rand;

#[inline(always)]
pub fn random_vec<R: Rng>(rng: &mut R, uniform: Uniform<f32>) -> Vec3A {
    Vec3A::new(
        uniform.sample(rng),
        uniform.sample(rng),
        uniform.sample(rng),
    )
}

#[inline(always)]
pub fn random_cosine_direction(rng: &mut Rand) -> Vec3A {
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = (1. - r2).sqrt();

    let phi = 2. * PI * r1;
    let sqrt_r2 = r2.sqrt();
    let x = phi.cos() * sqrt_r2;
    let y = phi.sin() * sqrt_r2;

    Vec3A::new(x, y, z)
}
