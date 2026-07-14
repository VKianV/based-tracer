use based_tracer::{color::write_color, config::Config, vec3::Vec3};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let config = Config::load(".env").expect("Failed to load config");
    let file = File::create(&config.output_name).expect("Couldn't create output file");

    let mut out = BufWriter::new(&file);

    writeln!(out, "P3").expect("couldnt' write p3");
    writeln!(out, "{} {}", &config.image_width, &config.image_height)
        .expect(" couldn't write width and height header");
    writeln!(out, "255").expect("couldn't write max colors in the header");

    print!("\x1b[?25lScanlines remaining: ");
    for hght_indx in 0..config.image_height {
        for wdth_indx in 0..config.image_width {
            write_color(
                &mut out,
                Vec3::new(
                    wdth_indx as f64 / (&config.image_width - 1) as f64,
                    hght_indx as f64 / (&config.image_height - 1) as f64,
                    0.0,
                ),
            );
            print!("\x1b[21G\x1b[K {}", &config.image_height - hght_indx);
        }
    }
    println!("\n\x1b[?25hDone!");
}
