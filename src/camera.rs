use glam::Vec3A;
use rand::Rng;

use crate::{ray::Ray, Rand};

fn random_in_unit_disk(rng: &mut Rand) -> Vec3A {
    let unit = Vec3A::new(1.0, 1.0, 0.0);
    loop {
        let p = 2.0 * Vec3A::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - unit;
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Vec3A,
    lower_left_corner: Vec3A,
    horizontal: Vec3A,
    vertical: Vec3A,
    u: Vec3A,
    v: Vec3A,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3A,
        look_at: Vec3A,
        view_up: Vec3A,
        vertical_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = focus_dist * f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - focus_dist * w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32, rng: &mut Rand) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
