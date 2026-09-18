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

use crate::mods::{
    app_error::AppError,
    color::{ray_color, write_color},
    config::Config,
    ray::Ray,
    shapes::hittable_list::HittableList,
    vec3::{Point3, Vec3},
};

pub struct CameraBuilder {
    output_name: String,
    image_width_pixels: usize,
    image_height_pixels: usize,
    focal_length: f64,
    vfov: f64,
    viewport_height_pixel: f64,
    samples_per_pixel: usize,
    max_depth: usize,
    num_threads: Option<usize>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            output_name: "render.ppm".to_string(),
            image_width_pixels: 1920,
            image_height_pixels: 1080,
            focal_length: 1.0,
            vfov: 90.0,
            viewport_height_pixel: 2.0,
            samples_per_pixel: 100,
            max_depth: 50,
            num_threads: None,
        }
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_config(path: &str) -> Self {
        let mut defaults = Self::default();

        // Whole file missing / unreadable → all defaults.
        let Ok(config) = Config::load_config(path) else {
            return defaults;
        };

        // Per-key fallback: keep the default if the key is absent or bad.
        if let Ok(v) = config.get_usize("image_width_pixels") {
            defaults.image_width_pixels = v;
        }
        if let Ok(v) = config.get_usize("image_height_pixels") {
            defaults.image_height_pixels = v;
        }
        if let Ok(v) = config.get_f64("focal_length") {
            defaults.focal_length = v;
        }
        if let Ok(v) = config.get_f64("viewport_height") {
            defaults.viewport_height_pixel = v;
        }
        if let Ok(v) = config.get_string("output_name") {
            defaults.output_name = v.to_string();
        }
        if let Ok(v) = config.get_usize("samples_per_pixel") {
            defaults.samples_per_pixel = v;
        }
        if let Ok(v) = config.get_usize("max_depth") {
            defaults.max_depth = v;
        }
        if let Ok(v) = config.get_usize("num_threads") {
            defaults.num_threads = Some(v);
        }

        defaults
    }

    pub fn output_name(mut self, name: impl Into<String>) -> Self {
        self.output_name = name.into();
        self
    }

    pub fn image_width_pixels(mut self, n: usize) -> Self {
        self.image_width_pixels = n;
        self
    }

    pub fn image_height_pixels(mut self, n: usize) -> Self {
        self.image_height_pixels = n;
        self
    }

    pub fn focal_length(mut self, f: f64) -> Self {
        self.focal_length = f;
        self
    }

    pub fn vfov(mut self, deg: f64) -> Self {
        self.vfov = deg;
        self
    }

    pub fn viewport_height_pixel(mut self, h: f64) -> Self {
        self.viewport_height_pixel = h;
        self
    }

    pub fn samples_per_pixel(mut self, n: usize) -> Self {
        self.samples_per_pixel = n;
        self
    }

    pub fn max_depth(mut self, n: usize) -> Self {
        self.max_depth = n;
        self
    }

    pub fn num_threads(mut self, n: usize) -> Self {
        self.num_threads = Some(n);
        self
    }

    pub fn build(self) -> Result<Camera, AppError> {
        // if self.image_width_pixels == 0 || self.image_height_pixels == 0 {
        //     return Err(AppError::invalid("image dimensions must be non-zero"));
        // }
        // if !(0.0..180.0).contains(&self.vfov) {
        //     return Err(AppError::invalid("vfov must be in (0, 180)"));
        // }

        // --- derived geometry ---
        let camera_center = Point3::ZERO;
        let focal_length = self.focal_length;

        let viewport_height_pixel = self.viewport_height_pixel;
        let viewport_width_pixel = viewport_height_pixel * self.image_width_pixels as f64
            / self.image_height_pixels as f64;

        let viewport_width = Point3::new(viewport_width_pixel, 0.0, 0.0);
        let viewport_height = Point3::new(0.0, -viewport_height_pixel, 0.0);

        let pixel_delta_width = viewport_width / self.image_width_pixels as f64;
        let pixel_delta_height = viewport_height / self.image_height_pixels as f64;

        let viewport_top_left = camera_center
            - Point3::new(0.0, 0.0, focal_length)
            - viewport_width / 2.0
            - viewport_height / 2.0;

        let top_left_pixel_position =
            viewport_top_left + 0.5 * (pixel_delta_width + pixel_delta_height);

        // --- derived thread count ---
        let num_threads = self.num_threads.unwrap_or_else(|| {
            thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
                .min(self.image_height_pixels)
        });

        Ok(Camera {
            output_name: self.output_name,
            image_width_pixels: self.image_width_pixels,
            image_height_pixels: self.image_height_pixels,
            num_threads,
            //derived
            pixel_delta_width,
            pixel_delta_height,
            camera_center,
            top_left_pixel_position,
        })
    }
}

pub struct Camera {
    // inputs (kept around for rendering)
    output_name: String,
    image_width_pixels: usize,
    image_height_pixels: usize,
    num_threads: usize,

    // derived
    pixel_delta_width: Vec3,
    pixel_delta_height: Vec3,
    camera_center: Point3,
    top_left_pixel_position: Point3,
}

impl Camera {
    pub fn render(&self, world: &HittableList) -> Result<(), AppError> {
        let start = Instant::now();

        // prepearing the output render
        let file = File::create(&self.output_name)?;
        let mut out = BufWriter::new(file);

        println!(
            "Rendering with {} threads (dynamic scheduling)...",
            &self.num_threads
        );

        print!("\x1b[?25l");
        print!("Scanlines remaining: {}", &self.image_height_pixels);
        const CHUNK_ROWS: usize = 8;

        let next_row = AtomicUsize::new(0);
        let (tx, rx) = mpsc::channel::<(usize, usize, Vec<u8>)>();

        thread::scope(|s| -> Result<(), AppError> {
            for _ in 0..self.num_threads {
                let tx = tx.clone();
                let next_row = &next_row;

                s.spawn(move || {
                    loop {
                        // One atomic operation per CHUNK_ROWS instead of per row.
                        let start_row = next_row.fetch_add(CHUNK_ROWS, Ordering::Relaxed);

                        if start_row >= self.image_height_pixels {
                            break;
                        }

                        let end_row = (start_row + CHUNK_ROWS).min(self.image_height_pixels);
                        let rows = end_row - start_row;

                        // One allocation for the whole chunk.
                        let mut chunk = Vec::with_capacity(rows * self.image_height_pixels * 3);

                        for h in start_row..end_row {
                            // Do the vertical multiplication once per row.
                            let mut pixel_center =
                                self.top_left_pixel_position + h as f64 * self.pixel_delta_height;

                            for _ in 0..self.image_width_pixels {
                                let ray_direction = pixel_center - self.camera_center;

                                let ray = Ray::new(self.camera_center, ray_direction);

                                write_color(&mut chunk, &ray_color(&ray, world))
                                    .expect("couldn't write color");

                                // Addition instead of w * pixel_delta_hor.
                                pixel_center += self.pixel_delta_width;
                            }
                        }

                        tx.send((start_row, end_row, chunk))
                            .expect("couldn't send data from workers");
                    }
                });
            }

            drop(tx);

            let num_chunks = self.image_height_pixels.div_ceil(CHUNK_ROWS);

            let mut chunks: Vec<Option<Vec<u8>>> = (0..num_chunks).map(|_| None).collect();

            let mut rows_received = 0;

            while rows_received < self.image_height_pixels {
                let (start_row, end_row, data) =
                    rx.recv().expect("couldn't recive data from workers");

                let chunk_idx = start_row / CHUNK_ROWS;
                chunks[chunk_idx] = Some(data);

                rows_received += end_row - start_row;

                let remaining = self.image_height_pixels - rows_received;
                print!("\x1b[21G\x1b[K {}", remaining);
            }

            writeln!(out, "P6")?;
            writeln!(
                out,
                "{} {}",
                self.image_width_pixels, self.image_height_pixels
            )?;
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
}
