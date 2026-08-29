use crate::{
    ray::Ray,
    vec3::{Point3, RGB},
};
use std::io::Write;

#[must_use]
pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = *center - r.origin();
    let a = r.direction().dot(r.direction());
    let h = r.direction().dot(oc);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = h * h - a * c;

    match discriminant {
        d if d < 0.0 => None,
        _ => Some((h - discriminant.sqrt()) / a ),
    }
}

const SPHERE_CENTER: Point3 = Point3::new(0.0, 0.0, -1.0);

#[must_use]
pub fn ray_color(r: &Ray) -> RGB {
    if let Some(t) = hit_sphere(&SPHERE_CENTER, 0.5, r) {
        let n = (r.at(t) - SPHERE_CENTER).unit();

        return 0.5 * RGB::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let a = 0.5 * (r.direction().unit().y() + 1.0);

    (1.0 - a) * RGB::new(1.0, 1.0, 1.0) + a * RGB::new(0.5, 0.7, 1.0)
}

pub fn write_color<W: Write>(out: &mut W, pixel: &RGB) -> std::io::Result<()> {
    let r = (255.999 * pixel.x()) as u8;
    let g = (255.999 * pixel.y()) as u8;
    let b = (255.999 * pixel.z()) as u8;

    out.write_all(&[r, g, b])
}
