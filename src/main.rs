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
    ops::Range,
    thread,
    time::Instant,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let load_time = Instant::now();

    // prepering the files and loading the config
    let config = Config::load_config(".env")?;
    let file = File::create(config.get_str("output_name")?)?;
    let threads_count = config.get_u32("threads")?;

    let image_height = config.get_u32("image_height")?;
    let image_width = config.get_u32("image_width")?;
    let image_size = image_width as usize * image_height as usize;

    // Camera
    let focal_length = config.get_f64("focal_length")?;
    let viewport_height = config.get_f64("viewport_height")?;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_hor = Point3::new(viewport_width, 0.0, 0.0);
    let viewport_ver = Point3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_hor = viewport_hor / image_width as f64;
    let pixel_delta_ver = viewport_ver / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Point3::new(0.0, 0.0, focal_length)
        - viewport_hor / 2.0
        - viewport_ver / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_hor + pixel_delta_ver);

    let loaded_in = load_time.elapsed().as_millis();

    println!("Data loaded in {}ms!", loaded_in);

    let calculation_time = Instant::now();

    // render

    let calculate_range = |range: Range<usize>| {
        range
            .into_iter()
            .map(|index| {
                let x = index % image_width as usize;
                let y = index / image_width as usize;

                let pixel_center =
                    pixel00_loc + x as f64 * pixel_delta_hor + y as f64 * pixel_delta_ver;

                let ray_direction = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_direction);

                ray_color(&ray)
            })
            .collect::<Vec<_>>()
    };

    let pixels = thread::scope(|scope| {
        let chunk_size = image_size.div_ceil(threads_count as usize);

        let mut handles = Vec::with_capacity(threads_count as usize);

        for thread_id in 0..threads_count as usize {
            let start = thread_id * chunk_size;
            let end = (start + chunk_size).min(image_size);

            if start >= image_size {
                break;
            }

            handles.push(scope.spawn(move || calculate_range(start..end)));
        }

        let mut pixels = Vec::with_capacity(image_size);

        for handle in handles {
            pixels.extend(handle.join().unwrap());
        }

        pixels
    });

    let calculated_in = calculation_time.elapsed().as_millis();

    println!("Data calculated in {}ms!", calculated_in);

    let write_time = Instant::now();

    let mut out = BufWriter::new(file);

    writeln!(out, "P6")?;
    writeln!(out, "{} {}", image_width, image_height)?;
    writeln!(out, "255")?;

    for pixel in pixels {
        write_color(&mut out, pixel)?;
    }

    let written_in = write_time.elapsed().as_millis();

    println!("Written in {}ms!", written_in);

    let total_time = loaded_in + calculated_in + written_in;

    println!("Done, program took {}ms!", total_time);

    Ok(())
}
