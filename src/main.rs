use based_tracer::{
    app_error::AppError,
    color::{color_to_bytes, ray_color},
    config::Config,
    ray::Ray,
    vec3::{Point3},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Instant,
    thread,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let start = Instant::now();
    let config = Config::load_config(".env")?;
    let file = File::create(config.get_str("output_name")?)?;

    let image_height = config.get_u32("image_height")? as usize;
    let image_width = config.get_u32("image_width")? as usize;
    let focal_length = config.get_f64("focal_length")?;
    let viewport_height = config.get_f64("viewport_height")?;
    let viewport_width =
        viewport_height * (config.get_f64("image_width")? / config.get_f64("image_height")?);

    let camera_center = Point3::zero();
    let viewport_hor = Point3::new(viewport_width, 0.0, 0.0);
    let viewport_ver = Point3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_hor = viewport_hor / config.get_f64("image_width")?;
    let pixel_delta_ver = viewport_ver / config.get_f64("image_height")?;
    let viewport_upper_left = camera_center
        - Point3::new(0.0, 0.0, focal_length)
        - viewport_hor / 2.0
        - viewport_ver / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_hor + pixel_delta_ver);

    let total_bytes = image_width * image_height * 3;
    let mut pixels = vec![0u8; total_bytes];

    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(image_height)
        .max(1);
    let rows_per_thread = image_height / num_threads;
    let remainder = image_height % num_threads;

    println!("Rendering with {num_threads} threads...");

    thread::scope(|scope| {
        let mut remaining_pixels: &mut [u8] = &mut pixels;
        let mut start_row = 0;

        for t in 0..num_threads {
            // Number of rows for this thread
            let rows = if t < remainder {
                rows_per_thread + 1
            } else {
                rows_per_thread
            };
            let bytes_for_thread = rows * image_width * 3;

            // Split the remaining pixels: this thread gets the first `bytes_for_thread` bytes,
            // and `remaining_pixels` becomes the rest for subsequent threads.
            let (thread_pixels, rest) = remaining_pixels.split_at_mut(bytes_for_thread);
            remaining_pixels = rest;

            // Capture immutable values needed by the thread
            let pixel00 = pixel00_loc;
            let d_hor = pixel_delta_hor;
            let d_ver = pixel_delta_ver;
            let cam_center = camera_center;
            let width = image_width;

            scope.spawn(move || {
                let mut offset = 0;
                for hght_indx in start_row..start_row + rows {
                    for wdth_indx in 0..width {
                        let pixel_center = pixel00
                            + (wdth_indx as f64 * d_hor)
                            + (hght_indx as f64 * d_ver);
                        let ray_direction = pixel_center - cam_center;
                        let r = Ray::new(cam_center, ray_direction);

                        let color = ray_color(&r);
                        let bytes = color_to_bytes(color);

                        thread_pixels[offset..offset + 3].copy_from_slice(&bytes);
                        offset += 3;
                    }
                }
            });

            start_row += rows;
        }
    });

    // Write the image
    let mut out = BufWriter::new(file);
    writeln!(out, "P6")?;
    writeln!(out, "{} {}", image_width, image_height)?;
    writeln!(out, "255")?;
    out.write_all(&pixels)?;

    println!("Done in {}ms!", start.elapsed().as_millis());

    Ok(())
}
