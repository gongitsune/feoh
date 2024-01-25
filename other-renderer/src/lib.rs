use anyhow::{Ok, Result};
use image::{DynamicImage, RgbImage};

pub struct RenderInfo {
    pub img_width: usize,
    pub img_height: usize,
}

fn render(info: &RenderInfo, pixels: &mut [u8]) -> Result<()> {
    let img_width = info.img_width;
    let img_height = info.img_height;

    for j in (0..img_height - 1).rev() {
        for i in 0..img_width {
            let r = i as f64 / (img_width - 1) as f64;
            let g = j as f64 / (img_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            let pix_idx = 3 * (j * img_width + i);
            pixels[pix_idx] = ir;
            pixels[pix_idx + 1] = ig;
            pixels[pix_idx + 2] = ib;
        }
    }

    Ok(())
}

pub fn output(info: &RenderInfo) -> Result<DynamicImage> {
    let mut pixels = vec![0; 3 * info.img_width * info.img_height];
    render(info, &mut pixels)?;

    let image = RgbImage::from_raw(info.img_width as u32, info.img_height as u32, pixels)
        .expect("image creation failed");

    Ok(DynamicImage::ImageRgb8(image))
}
