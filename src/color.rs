use crate::{ray::Ray, vec3::RGB};
use std::io::{Result, Write};

pub fn ray_color(r: &Ray) -> RGB {
    let unit_direction = r.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * RGB::new(1.0, 1.0, 1.0) + a * RGB::new(0.5, 0.7, 1.0)
}

pub fn write_color<W: Write>(out: &mut W, pixel: RGB) -> Result<()> {
    writeln!(
        out,
        "{} {} {}",
        (255.999 * pixel.x()) as u8,
        (255.999 * pixel.y()) as u8,
        (255.999 * pixel.z()) as u8
    )?;

    Ok(())
}
