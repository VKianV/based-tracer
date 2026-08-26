use based_tracer::{
    app_error::AppError,
    color::{ray_color, write_color},
    config::Config,
    ray::Ray,
    vec3::{Point3, RGB},
};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufWriter,Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
        Arc,
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

    // Load config and create output file
    let config = Config::load_config(".env")?;
    let image_height = config.get_u32("image_height")?;
    let image_width = config.get_u32("image_width")?;
    let file = File::create(config.get_str("output_name")?)?;

    // Camera setup (same as before)
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

    // Determine number of threads
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(image_height as usize)
        .max(1);

    // Bounded channel to limit how many rows can be in flight.
    // This prevents workers from producing too much data before the writer can consume it.
    let channel_bound = num_threads * 2; // adjust if needed
    let (tx, rx) = mpsc::sync_channel::<(usize, Vec<RGB>)>(channel_bound);

    // Spawn the writer thread
    let writer_handle = thread::spawn(move || -> Result<(), AppError> {
        let mut out = BufWriter::new(file);
        let mut buffer: BTreeMap<usize, Vec<RGB>> = BTreeMap::new();
        let mut next_row_to_write = 0usize;

        // Receive rows and write them in order
        while let Ok((row_index, row_colors)) = rx.recv() {
            buffer.insert(row_index, row_colors);

            // Write all consecutive rows that are now available
            while let Some(colors) = buffer.remove(&next_row_to_write) {
                for color in colors {
                    write_color(&mut out, color)?;
                }
                next_row_to_write += 1;
            }
        }

        // Channel closed – write any remaining rows (shouldn't happen, but just in case)
        while let Some(colors) = buffer.remove(&next_row_to_write) {
            for color in colors {
                write_color(&mut out, color)?;
            }
            next_row_to_write += 1;
        }

        out.flush()?;
        Ok(())
    });

    // Atomic counter for dynamic row distribution
    let next_row = Arc::new(AtomicUsize::new(0));
    let mut worker_handles = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        // Clone the sender for this worker
        let tx = tx.clone();
        let next_row = Arc::clone(&next_row);

        // Move necessary data into the closure (they are Copy or cloneable)
        let handle = thread::spawn(move || {
            loop {
                // Fetch the next row index
                let row = next_row.fetch_add(1, Ordering::Relaxed);
                if row >= image_height as usize {
                    break;
                }

                // Compute colors for this row
                let mut row_colors = Vec::with_capacity(image_width as usize);
                for col in 0..image_width as usize {
                    let pixel_center = pixel00_loc
                        + (col as f64 * pixel_delta_hor)
                        + (row as f64 * pixel_delta_ver);
                    let ray_direction = pixel_center - camera_center;
                    let r = Ray::new(camera_center, ray_direction);
                    let color = ray_color(&r);
                    row_colors.push(color);
                }

                // Send the row to the writer; if send fails, the writer has died
                if tx.send((row, row_colors)).is_err() {
                    break;
                }
            }
        });

        worker_handles.push(handle);
    }

    // Drop the original sender so the channel closes when all workers finish
    drop(tx);

    // Wait for all workers to finish
    for handle in worker_handles {
        let _ = handle.join();
    }

    // Wait for the writer thread to finish and get its result
    writer_handle.join().unwrap()?;

    println!("\nDone in {}ms!", start.elapsed().as_millis());
    Ok(())
}
