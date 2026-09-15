use crate::{
    constants::{BLUE_COLOR, WHITE_COLOR},
    ray::Ray,
    shapes::hittable::Hittable,
    vec3::RGB,
};
use std::io::{self, Write};

#[must_use]
pub fn ray_color(ray: &Ray, world: &impl Hittable) -> RGB {
    if let Some(hit_record) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (hit_record.normal + WHITE_COLOR);
    }

    let a = 0.5 * (ray.direction().unit().y() + 1.0);

    (1.0 - a) * WHITE_COLOR + a * BLUE_COLOR
}

#[inline]
pub fn write_color<W: Write>(output: &mut W, pixel: &RGB) -> io::Result<()> {
    output.write_all(&[
        (255.999 * pixel.x()) as u8,
        (255.999 * pixel.y()) as u8,
        (255.999 * pixel.z()) as u8,
    ])
}
