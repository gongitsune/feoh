pub mod checker_texture;
pub mod solid_color;

use glam::Vec3A;

pub trait Texture: Sync {
    fn value(&self, u: f32, v: f32, p: &Vec3A) -> Vec3A;
}
