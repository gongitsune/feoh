use glam::Vec3A;
use rand::{distributions::Uniform, Rng};

use crate::{
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    vec::random_vec,
    Rand,
};

pub fn random_scene(rng: &mut Rand) -> HittableList {
    let mut world = HittableList::default();

    let ground_mat = Lambertian::new(Vec3A::splat(0.5));
    world.push(Sphere::new(
        Vec3A::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3A::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = {
                        let a = random_vec(rng, Uniform::new(0., 1.));
                        let b = random_vec(rng, Uniform::new(0., 1.));
                        Vec3A::new(a.x * b.x, a.y * b.y, a.z * b.z)
                    };
                    world.push(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec(rng, Uniform::new(0.5, 1.));
                    let fuzzy = rng.gen_range(0.0..=0.5);
                    world.push(Sphere::new(center, 0.2, Metal::new(albedo, fuzzy)));
                } else {
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.push(Sphere::new(Vec3A::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Lambertian::new(Vec3A::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(Vec3A::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Vec3A::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(Vec3A::new(4.0, 1.0, 0.0), 1.0, mat3));

    world
}
