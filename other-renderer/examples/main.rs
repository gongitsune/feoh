use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let info = other_renderer::RenderInfo {
        img_width: 256,
        img_height: 256,
    };
    let img = other_renderer::output(&info)?;

    img.save("dist/output.png")?;

    Ok(())
}
