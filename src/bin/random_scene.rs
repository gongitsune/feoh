use std::{fs::OpenOptions, io::BufWriter, path::Path};

use anyhow::{Ok, Result};
use feoh::draw;

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(Path::new("image.ppm"))?;
    let mut writer = BufWriter::new(file);

    draw(512, 1024, 1024, 50, &mut writer)?;

    Ok(())
}
