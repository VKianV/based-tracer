use based_tracer::mods::{
    app_error::AppError,
    camera::CameraBuilder,
    constants::{GROUND_CENTER, SPHERE_CENTER},
    shapes::{hittable::Shapes, hittable_list::HittableList, sphere::Sphere},
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    // adding items to the world
    let mut world = HittableList::new();
    world.add(Shapes::Sphere(Sphere::new(SPHERE_CENTER, 0.5)));
    world.add(Shapes::Sphere(Sphere::new(GROUND_CENTER, 100.0)));

    CameraBuilder::from_config("config.env")
        .build()?
        .render(&world)?;

    Ok(())
}
