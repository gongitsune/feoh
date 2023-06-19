use glam::Vec3A;

use super::{solid_color::SolidColor, Texture};

pub struct CheckerTexture<Odd: Texture + Sync, Even: Texture + Sync> {
    pub odd: Odd,
    pub even: Even,
}

impl<Odd: Texture + Sync, Even: Texture + Sync> CheckerTexture<Odd, Even> {
    #[allow(dead_code)]
    pub fn new(odd: Odd, even: Even) -> Self {
        Self { odd, even }
    }
}

impl From<(Vec3A, Vec3A)> for CheckerTexture<SolidColor, SolidColor> {
    fn from((odd, even): (Vec3A, Vec3A)) -> Self {
        Self {
            odd: SolidColor::new(odd),
            even: SolidColor::new(even),
        }
    }
}

impl<Odd: Texture + Sync, Even: Texture + Sync> Texture for CheckerTexture<Odd, Even> {
    fn value(&self, u: f32, v: f32, p: &glam::Vec3A) -> glam::Vec3A {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
