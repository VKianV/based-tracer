use based_tracer::{
    color::{ray_color, write_color},
    config::Config,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

fn main() -> io::Result<()> {
    let config = Config::load(".env")?;

    let file = File::create(&config.output_name)?;
    let mut out = BufWriter::new(&file);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (config.image_width as f64 / config.image_height as f64);
    let camera_center = Point3::zero();

    // Viewport vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Pixel delta vectors
    let pixel_delta_u = viewport_u / config.image_width as f64;
    let pixel_delta_v = viewport_v / config.image_height as f64;

    // Upper‑left pixel location
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", &config.image_width, &config.image_height)?;
    writeln!(out, "255")?;

    print!("\x1b[?25lScanlines remaining: ");
    for hght_indx in 0..config.image_height {
        for wdth_indx in 0..config.image_width {
            // write_color(
            //     &mut out,
            //     Vec3::new(
            //         wdth_indx as f64 / (&config.image_width - 1) as f64,
            //         hght_indx as f64 / (&config.image_height - 1) as f64,
            //         0.0,
            //     ),
            // )?;

            let pixel_center = pixel00_loc
                + (wdth_indx as f64 * pixel_delta_u)
                + (hght_indx as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut out, pixel_color)?;

            // let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;

            // let ray_direction = pixel_center - camera_center;

            // let ray = Ray::new(camera_center, ray_direction);

            // let pixel_color = ray_color(&ray);

            // write_color(&mut out, &pixel_color);

            print!("\x1b[21G\x1b[K {}", &config.image_height - hght_indx);
        }
    }

    println!("\n\x1b[?25hDone!");

    Ok(())
}
