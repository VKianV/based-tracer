use crate::{
    ray::Ray,
    shapes::hittable::Hittable,
    utils::{BLUE_COLOR, WHITE_COLOR},
    vec3::RGB,
};
use std::io::{self, Write};

// #[must_use]
// pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
//     let oc = *center - r.origin();
//     let a = r.direction().dot(r.direction());
//     let h = r.direction().dot(oc);
//     let c = oc.length_squared() - radius.powi(2);
//     let discriminant = h.powi(2) - a * c;

//     match discriminant {
//         d if d < 0.0 => None,
//         _ => Some((h - discriminant.sqrt()) / a),
//     }
// }

#[must_use]
pub fn ray_color(r: &Ray, world: &dyn Hittable) -> RGB {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + WHITE_COLOR);
    }
    // if let Some(t) = hit_sphere(&SPHERE_CENTER, 0.5, r) {
    //     let n = (r.at(t) - SPHERE_CENTER).unit();

    //     return 0.5 * RGB::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    // }

    let a = 0.5 * (r.direction().unit().y() + 1.0);

    (1.0 - a) * WHITE_COLOR + a * BLUE_COLOR
}

#[inline]
pub fn write_color<W: Write>(out: &mut W, pixel: &RGB) -> io::Result<()> {
    out.write_all(&[
        (255.999 * pixel.x()) as u8,
        (255.999 * pixel.y()) as u8,
        (255.999 * pixel.z()) as u8,
    ])
}
