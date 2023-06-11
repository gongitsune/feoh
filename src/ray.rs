use glam::Vec3A;

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[inline(always)]
    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }
}
