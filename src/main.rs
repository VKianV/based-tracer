use based_tracer::{
    app_error::AppError,
    color::{ray_color, write_color},
    config::Config,
    ray::Ray,
    vec3::Point3,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Instant,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let start = Instant::now();
    // prepering the files and loading the config
    let config = Config::load_config(".env")?;
    let file = File::create(config.get_str("output_name")?)?;
    let mut out = BufWriter::new(&file);

    let image_height = config.get_f64("image_height")?;
    let image_width = config.get_f64("image_width")?;

    // Camera
    let focal_length = config.get_f64("focal_length")?;
    let viewport_height = config.get_f64("viewport_height")?;
    let viewport_width = viewport_height * (image_width / image_height);
    let camera_center = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_hor = Point3::new(viewport_width, 0.0, 0.0);
    let viewport_ver = Point3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_hor = viewport_hor / image_width;
    let pixel_delta_ver = viewport_ver / image_height;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Point3::new(0.0, 0.0, focal_length)
        - viewport_hor / 2.0
        - viewport_ver / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_hor + pixel_delta_ver);

    // render
    writeln!(out, "P3")?;
    writeln!(out, "{} {}", image_width, image_height)?;
    writeln!(out, "255")?;

    print!("\x1b[?25lScanlines remaining: ");
    for hght_indx in 0..(image_height as u32) {
        for wdth_indx in 0..(image_width as u32) {
            let pixel_center = pixel00_loc
                + (wdth_indx as f64 * pixel_delta_hor)
                + (hght_indx as f64 * pixel_delta_ver);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            write_color(&mut out, ray_color(&r))?;
        }
        print!("\x1b[21G\x1b[K {}", image_height as u32 - hght_indx);
    }

    println!("\n\x1b[?25hDone in {}ms!", start.elapsed().as_millis());

    Ok(())
}
