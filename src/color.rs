use crate::{
    ray::Ray,
    vec3::{Point3, RGB},
};
use std::io::{Result, Write};

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = center - r.origin();
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b * b - 4.0 * a * c;

    match discriminant {
        d if d < 0.0 => None,
        _ => Some((-b - discriminant.sqrt()) / (2.0 * a)),
    }
}

pub fn ray_color(r: &Ray) -> RGB {
    // Compute surface normal at hit point and map to color
    if let Some(t) = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        let n = (r.at(t) - Point3::new(0.0, 0.0, -1.0)).unit();

        return 0.5 * RGB::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    // Compute backround color
    let a = 0.5 * (r.direction().unit().y() + 1.0);

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
