use glam::Vec3A;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

#[inline(always)]
pub fn random_vec<R: Rng>(rng: &mut R, uniform: Uniform<f32>) -> Vec3A {
    Vec3A::new(
        uniform.sample(rng),
        uniform.sample(rng),
        uniform.sample(rng),
    )
}
