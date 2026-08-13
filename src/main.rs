use based_tracer::{
    app_error::AppError,
    color::{ray_color, write_color},
    config::Config,
    ray::Ray,
    vec3::{Point3},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let config = Config::load_config(".env")?;
    let file = File::create(config.get_str("output_name")?)?;
    let mut out = BufWriter::new(&file);

    // Camera
    let focal_length = config.get_f64("focal_length")?;
    let viewport_height = config.get_f64("viewport_height")?;
    let viewport_width = viewport_height
        * (config.get_f64("image_width")? / config.get_f64("image_height")?);
    let camera_center = Point3::zero();

    let viewport_hor = Point3::new(viewport_width, 0.0, 0.0);
    let viewport_ver = Point3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_hor = viewport_hor / config.get_f64("image_width")?;
    let pixel_delta_ver = viewport_ver / config.get_f64("image_height")?;

    let viewport_upper_left =
        camera_center - Point3::new(0.0, 0.0, focal_length) - viewport_hor / 2.0 - viewport_ver / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_hor + pixel_delta_ver);

    writeln!(out, "P3")?;
    writeln!(
        out,
        "{} {}",
        config.get_u32("image_width")?,
        config.get_u32("image_height")?
    )?;
    writeln!(out, "255")?;

    print!("\x1b[?25lScanlines remaining: ");
    for hght_indx in 0..config.get_u32("image_height")? {
        for wdth_indx in 0..config.get_u32("image_width")? {
            let pixel_center = pixel00_loc
                + (wdth_indx as f64 * pixel_delta_hor)
                + (hght_indx as f64 * pixel_delta_ver);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            write_color(&mut out, ray_color(&r))?;

            print!(
                "\x1b[21G\x1b[K {}",
                config.get_u32("image_height")? - hght_indx
            );
        }
    }

    println!("\n\x1b[?25hDone!");
    Ok(())
}
