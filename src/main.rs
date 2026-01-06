use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Image
    let image_width: u32 = 1280;
    let image_height: u32 = 720;

    // Render header (PPM P3 format - ASCII)
    let stdout = io::stdout();
    let mut out = stdout.lock();

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", image_width, image_height)?;
    writeln!(out, "255")?;

    // Render pixels
    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            // The classic 255.999 trick to avoid rounding issues at exactly 1.0
            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            writeln!(out, "{} {} {}", ir, ig, ib)?;
        }
    }

    out.flush()?;
    println!("Done. Image written to stdout (PPM format).");
    Ok(())
}
