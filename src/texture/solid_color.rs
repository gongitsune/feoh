use super::Texture;
use glam::Vec3A;

#[derive(Clone)]
pub struct SolidColor {
    color: Vec3A,
}

impl SolidColor {
    pub fn new(color: Vec3A) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f32, _: f32, _: &Vec3A) -> Vec3A {
        self.color
    }
}
