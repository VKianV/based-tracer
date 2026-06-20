// color.rs
use crate::vec3::Color;
use std::io::Write;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Convert [0,1] to [0,255]
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}
