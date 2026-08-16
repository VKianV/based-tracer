use crate::{
    ray::Ray,
    vec3::{Point3, RGB},
};
use std::io::{Result, Write};

// --- Sphere hit test ---
pub fn hit_sphere(center: RGB, radius: f64, r: &Ray) -> bool {
    let oc = center - r.origin();
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius.powi(2);

    b * b - 4.0 * a * c >= 0.0
}

pub fn ray_color(r: &Ray) -> RGB {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return RGB::new(0.0, 0.0, 1.0);
    }

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
