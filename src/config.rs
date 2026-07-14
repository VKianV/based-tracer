use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader},
};

pub struct Config {
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub vfov: f64,
    pub output_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            image_width: 256,
            image_height: 256,
            samples_per_pixel: 100,
            max_depth: 50,
            vfov: 20.0,
            output_name: "render.ppm".to_string(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> io::Result<Self> {
        let mut cfg = Config::default();

        let contents = fs::read_to_string(path)?;

        for line in contents.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            match key.trim() {
                "image_width" => cfg.image_width = value.trim().parse().unwrap(),
                "image_height" => cfg.image_height = value.trim().parse().unwrap(),
                "samples_per_pixel" => cfg.samples_per_pixel = value.trim().parse().unwrap(),
                "max_depth" => cfg.max_depth = value.trim().parse().unwrap(),
                "vfov" => cfg.vfov = value.trim().parse().unwrap(),
                "output_name" => cfg.output_name = value.trim().to_string(),
                _ => {}
            }
        }

        Ok(cfg)
    }
}

pub fn load_config(path: &str) -> io::Result<HashMap<String, String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Split at first '='
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();
            map.insert(key, value);
        }
    }

    Ok(map)
}

pub fn get_u32(map: &HashMap<String, String>, key: &str, default: u32) -> u32 {
    map.get(key).and_then(|s| s.parse().ok()).unwrap_or(default)
}

pub fn get_string<'a>(map: &'a HashMap<String, String>, key: &str, default: &'a str) -> &'a str {
    map.get(key).map(|s| s.as_str()).unwrap_or(default)
}
