use std::sync::Arc;

use crate::{
    hittable::{
        boxtype::BoxType,
        flip_face::FlipFace,
        hittable_list::HittableList,
        moving_sphere::MovingSphere,
        rect::{AARect, Plane},
        rotate::{Axis, Rotate},
        sphere::Sphere,
        translate::Translate,
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    texture::checker_texture::CheckerTexture,
    vec::random_vec,
    Rand,
};
use glam::Vec3A;
use rand::{distributions::Uniform, Rng};

#[allow(dead_code)]
pub fn random_scene(rng: &mut Rand) -> HittableList {
    let mut world = HittableList::default();

    let checker_tex = CheckerTexture::from((Vec3A::new(0.2, 0.3, 0.1), Vec3A::splat(0.9)));
    let ground_mat = Lambertian::new(checker_tex).into();
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
                    let center2 = center + Vec3A::new(0., rng.gen_range(0.0..0.5), 0.);
                    world.push(MovingSphere::new(
                        (center, center2),
                        (0., 1.),
                        0.2,
                        Lambertian::from(albedo).into(),
                    ));
                    // world.push(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec(rng, Uniform::new(0.5, 1.));
                    let fuzzy = rng.gen_range(0.0..=0.5);
                    world.push(Sphere::new(center, 0.2, Metal::new(albedo, fuzzy).into()));
                } else {
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5).into()));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5).into();
    world.push(Sphere::new(Vec3A::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Lambertian::from(Vec3A::new(0.4, 0.2, 0.1)).into();
    world.push(Sphere::new(Vec3A::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Vec3A::new(0.7, 0.6, 0.5), 0.0).into();
    world.push(Sphere::new(Vec3A::new(4.0, 1.0, 0.0), 1.0, mat3));

    world
}

pub fn cornell_box() -> HittableList {
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::from(Vec3A::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(Vec3A::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(Vec3A::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(Vec3A::new(15., 15., 15.)));

    world.push(AARect::new(Plane::YZ, (0., 555.), (0., 555.), 555., green));
    world.push(AARect::new(Plane::YZ, (0., 555.), (0., 555.), 0., red));

    let light_rect = AARect::new(Plane::XZ, (213., 343.), (227., 332.), 554., light).into();
    world.push(FlipFace::new(light_rect));
    world.push(AARect::new(
        Plane::XZ,
        (0., 555.),
        (0., 555.),
        0.,
        white.clone(),
    ));
    world.push(AARect::new(
        Plane::XZ,
        (0., 555.),
        (0., 555.),
        555.,
        white.clone(),
    ));
    world.push(AARect::new(
        Plane::XY,
        (0., 555.),
        (0., 555.),
        555.,
        white.clone(),
    ));

    world.push({
        let instance = BoxType::new((Vec3A::ZERO, Vec3A::new(165., 330., 165.)), white.clone());
        let instance = Rotate::new(Axis::Y, Arc::new(instance), 15.);
        let instance = Translate::new(Arc::new(instance), Vec3A::new(265., 0., 295.));

        instance
    });
    world.push({
        let instance = BoxType::new((Vec3A::ZERO, Vec3A::new(165., 165., 165.)), white);
        let instance = Rotate::new(Axis::Y, Arc::new(instance), -18.);
        let instance = Translate::new(Arc::new(instance), Vec3A::new(130., 0., 65.));

        instance
    });

    world
}
