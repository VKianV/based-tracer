use based_tracer::{
    app_error::AppError,
    color::{ray_color, write_color},   // removed color_to_bytes
    config::Config,
    ray::Ray,
    vec3::Point3,
};

use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
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

    let config = Config::load_config(".env")?;
    let image_width = config.get_u32("image_width")?;
    let image_height = config.get_u32("image_height")?;
    let focal_length = config.get_f64("focal_length")?;
    let viewport_height = config.get_f64("viewport_height")?;
    let output_name = config.get_str("output_name")?;

    let file = File::create(output_name)?;
    let mut out = BufWriter::new(file);

    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Point3::zero();

    let viewport_hor = Point3::new(viewport_width, 0.0, 0.0);
    let viewport_ver = Point3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_hor = viewport_hor / image_width as f64;
    let pixel_delta_ver = viewport_ver / image_height as f64;

    let viewport_upper_left = camera_center
        - Point3::new(0.0, 0.0, focal_length)
        - viewport_hor / 2.0
        - viewport_ver / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_hor + pixel_delta_ver);

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(image_height as usize);

    let chunk_size = 4_usize;
    let total_chunks = (image_height as usize + chunk_size - 1) / chunk_size;

    let next_chunk = AtomicUsize::new(0);
    let (tx, rx) = mpsc::channel::<(usize, Vec<u8>)>();

    println!("Rendering with {num_threads}");

    print!("\x1b[?25l");
    print!("Scanlines remaining: {image_height}");

    let scope_result: Result<(), AppError> = std::thread::scope(|s| {
        for _ in 0..num_threads {
            let tx = tx.clone();
            let next_chunk = &next_chunk;
     
            s.spawn(move || {
                loop {
                    let chunk_idx = next_chunk.fetch_add(1, Ordering::Relaxed);
                    if chunk_idx >= total_chunks {
                        break;
                    }

                    let start_row = chunk_idx * chunk_size;
                    let end_row = ((chunk_idx + 1) * chunk_size).min(image_height as usize);

                    for h in start_row..end_row {
                        let mut row_bytes = Vec::with_capacity(image_width as usize * 3);
                        for w in 0..image_width as usize {
                            let pixel_center = pixel00_loc
                                + (w as f64 * pixel_delta_hor)
                                + (h as f64 * pixel_delta_ver);
                            let ray_direction = pixel_center - camera_center;
                            let r = Ray::new(camera_center, ray_direction);

                            write_color(&mut row_bytes, ray_color(&r)).unwrap();
                        }
                        tx.send((h, row_bytes)).unwrap();
                    }
                }
            });
        }

        let mut rows: Vec<Option<Vec<u8>>> = vec![None; image_height as usize];
        let mut received = 0_usize;

        while received < image_height as usize {
            if let Ok((row_idx, data)) = rx.recv() {
                rows[row_idx] = Some(data);
                received += 1;

                let remaining = image_height as usize - received;
                print!("\x1b[21G\x1b[K {}", remaining);
                std::io::stdout().flush().unwrap();
            }
        }

        writeln!(out, "P6")?;
        writeln!(out, "{} {}", image_width, image_height)?;
        writeln!(out, "255")?;

        for row_data in rows.into_iter().flatten() {
            out.write_all(&row_data)?;
        }

        Ok(())
    });

    scope_result?;

    out.flush()?;

    println!("\x1b[?25h");
    println!("Done in {}ms!", start.elapsed().as_millis());

    Ok(())
}
