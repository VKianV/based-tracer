use crate::mods::{
    interval::Interval,
    ray::Ray,
    shapes::sphere::Sphere,
    vec3::{Point3, Vec3},
};

#[derive(PartialEq)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub distance: f64,
    pub front_face: bool,
}

impl HitRecord {
    #[must_use]
    pub const fn new(point: Point3, normal: Vec3, distance: f64, front_face: bool) -> Self {
        Self {
            point,
            normal,
            distance,
            front_face,
        }
    }

    /// Sets the hit record normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_and_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_distance_interval: Interval) -> Option<HitRecord>;
}

pub enum Shapes {
    Sphere(Sphere),
}

impl Hittable for Shapes {
    fn hit(&self, ray: &Ray, ray_distance_interval: Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, ray_distance_interval),
            // Self::Plane(p) => p.hit(ray, ray_t),
        }
    }
}
