use based_tracer::{
    app_error::AppError,
    color::{ray_color, write_color},
    config::Config,
    constants::{GROUND_CENTER, SPHERE_CENTER},
    ray::Ray,
    shapes::{
        hittable::{HittableList, Shapes},
        sphare::Sphare,
    },
    vec3::Point3,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
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
    let start = Instant::now();

    // importing the config data for initialization
    let config = Config::load_config("config.env")?;
    let image_width_f64 = config.get_f64("image_width")?;
    let image_height_f64 = config.get_f64("image_height")?;
    let image_width = config.get_usize("image_width")?;
    let image_height = config.get_usize("image_height")?;
    let focal_length = config.get_f64("focal_length")?;
    let viewport_height = config.get_f64("viewport_height")?;
    let output_name = config.get_string("output_name")?;

    // prepearing the output render
    let file = File::create(output_name)?;
    let mut out = BufWriter::new(file);

    // adding items to the world
    let mut world = HittableList::new();
    world.add(Shapes::Sphare(Sphare::new(SPHERE_CENTER, 0.5)));
    world.add(Shapes::Sphare(Sphare::new(GROUND_CENTER, 100.0)));

    // prepearing the viewport and the camera and justifications for pixel placement
    let viewport_width = viewport_height * image_width_f64 / image_height_f64;
    let camera_center = Point3::zero();

    let viewport_width = Point3::new(viewport_width, 0.0, 0.0);
    let viewport_height = Point3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_width = viewport_width / image_width_f64;
    let pixel_delta_height = viewport_height / image_height_f64;

    let viewport_top_left = camera_center
        - Point3::new(0.0, 0.0, focal_length)
        - viewport_width / 2.0
        - viewport_height / 2.0;

    let top_left_pixel_position =
        viewport_top_left + 0.5 * (pixel_delta_width + pixel_delta_height);

    // prepearing the hotloop for threads
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(image_height);

    println!("Rendering with {num_threads} threads (dynamic scheduling)...");

    print!("\x1b[?25l");
    print!("Scanlines remaining: {image_height}");
    const CHUNK_ROWS: usize = 8;

    let next_row = AtomicUsize::new(0);
    let (tx, rx) = mpsc::channel::<(usize, usize, Vec<u8>)>();

    thread::scope(|s| -> Result<(), AppError> {
        for _ in 0..num_threads {
            let tx = tx.clone();
            let next_row = &next_row;
            let world = &world;

            s.spawn(move || {
                loop {
                    // One atomic operation per CHUNK_ROWS instead of per row.
                    let start_row = next_row.fetch_add(CHUNK_ROWS, Ordering::Relaxed);

                    if start_row >= image_height {
                        break;
                    }

                    let end_row = (start_row + CHUNK_ROWS).min(image_height);
                    let rows = end_row - start_row;

                    // One allocation for the whole chunk.
                    let mut chunk = Vec::with_capacity(rows * image_width * 3);

                    for h in start_row..end_row {
                        // Do the vertical multiplication once per row.
                        let mut pixel_center =
                            top_left_pixel_position + h as f64 * pixel_delta_height;

                        for _ in 0..image_width {
                            let ray_direction = pixel_center - camera_center;

                            let ray = Ray::new(camera_center, ray_direction);

                            write_color(&mut chunk, &ray_color(&ray, world)).unwrap();

                            // Addition instead of w * pixel_delta_hor.
                            pixel_center += pixel_delta_width;
                        }
                    }

                    tx.send((start_row, end_row, chunk)).unwrap();
                }
            });
        }

        drop(tx);

        let num_chunks = image_height.div_ceil(CHUNK_ROWS);

        let mut chunks: Vec<Option<Vec<u8>>> = (0..num_chunks).map(|_| None).collect();

        let mut rows_received = 0;

        while rows_received < image_height {
            let (start_row, end_row, data) = rx.recv().unwrap();

            let chunk_idx = start_row / CHUNK_ROWS;
            chunks[chunk_idx] = Some(data);

            rows_received += end_row - start_row;

            let remaining = image_height - rows_received;
            print!("\x1b[21G\x1b[K {}", remaining);
        }

        writeln!(out, "P6")?;
        writeln!(out, "{image_width} {image_height}")?;
        writeln!(out, "255")?;

        for chunk in chunks.into_iter().flatten() {
            out.write_all(&chunk)?;
        }

        Ok(())
    })?;

    println!("\x1b[?25h");
    println!("Done in {:.3}s!", start.elapsed().as_secs_f64());

    Ok(())
}
