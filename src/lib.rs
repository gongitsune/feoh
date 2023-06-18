use crate::{camera::Camera, scene::random_scene};
use anyhow::Result;
use glam::Vec3A;
use hittable::{bvh::BvhTree, Hittable};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use ray::Ray;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use scene::cornell_box;
use std::{
    f32::INFINITY,
    io::{BufWriter, Write},
};

mod camera;
mod hittable;
mod material;
mod ray;
mod scene;
mod texture;
mod vec;

pub type Rand = SmallRng;

fn ray_color<H: Hittable>(
    ray: &Ray,
    background: &Vec3A,
    world: &H,
    depth: usize,
    rng: &mut Rand,
) -> Vec3A {
    if depth <= 0 {
        return Vec3A::ZERO;
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.point);
        if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit, rng) {
            let color = ray_color(&scattered, background, world, depth - 1, rng);
            Vec3A::new(
                emitted.x + attenuation.x * color.x,
                emitted.y + attenuation.y * color.y,
                emitted.z + attenuation.z * color.z,
            )
        } else {
            emitted
        }
    } else {
        *background
    }
}

pub fn draw<W: Write>(
    img_height: usize,
    img_width: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    writer: &mut BufWriter<W>,
) -> Result<()> {
    // Image
    let aspect_ratio = img_width as f32 / img_height as f32;

    // Progress
    let multi_pb = MultiProgress::new();
    let sub_pb_style =
        ProgressStyle::with_template("           ┣ {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg}")?
            .progress_chars("##-");
    let main_pb = multi_pb.add(ProgressBar::new(img_height as u64));
    main_pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] ┏ {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg}",
        )?
        .progress_chars("##-"),
    );

    // World
    let mut rng = SmallRng::from_entropy();
    let mut world = cornell_box();
    let world = BvhTree::new(&mut world.objects, (0., 1.), &mut rng);
    let background = Vec3A::ZERO;

    // Camera
    let look_from = Vec3A::new(278., 278., -800.);
    let look_at = Vec3A::new(278., 278., 0.);
    let focus_dist = (look_from - look_at).length();
    let aperture = 0.;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3A::new(0.0, 1.0, 0.0),
        40.0,
        aspect_ratio,
        aperture,
        focus_dist,
        (0., 1.),
    );

    // Render
    multi_pb.println("✨ Generating...")?;
    writeln!(writer, "P3\n{} {}\n255", img_width, img_height)?;
    let image = (0..img_height)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            main_pb.inc(1);
            let width_pb = multi_pb.add(ProgressBar::new(img_width as u64));
            width_pb.set_style(sub_pb_style.clone());
            let mut rng = SmallRng::from_entropy();
            (0..img_width)
                .flat_map(|x| {
                    width_pb.inc(1);
                    let scale = 1.0 / samples_per_pixel as f32;
                    (0..samples_per_pixel)
                        .map(|_| {
                            let u = (x as f32 + rng.gen::<f32>()) / (img_width - 1) as f32;
                            let v = (y as f32 + rng.gen::<f32>()) / (img_height - 1) as f32;

                            let ray = camera.get_ray(u, v, &mut rng);
                            ray_color(&ray, &background, &world, max_depth, &mut rng)
                        })
                        .sum::<Vec3A>()
                        .to_array()
                        .map(|c| (256.0 * (c * scale).sqrt().clamp(0.0, 0.999)) as u8)
                        .to_vec()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();
    for col in image.chunks(3) {
        writeln!(writer, "{} {} {}", col[0], col[1], col[2])?;
    }

    main_pb.abandon_with_message("Generated.");
    multi_pb.println("🍻 Done!!")?;

    Ok(())
}
