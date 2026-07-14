use crate::vec3::Vec3;
use std::io::Write;

pub fn write_color<W: Write>(out: &mut W, pixel: Vec3) {
    let r = (255.999 * pixel.x) as u8;
    let g = (255.999 * pixel.y) as u8;
    let b = (255.999 * pixel.z) as u8;

    writeln!(out, "{} {} {}", r, g, b).expect("couldn't write pixel color");
}
