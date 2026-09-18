use crate::mods::{
    interval::Interval,
    ray::Ray,
    shapes::hittable::{HitRecord, Hittable},
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    #[must_use]
    pub const fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_distance_interval: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_distance_interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_distance_interval.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);

        let mut hit_record = HitRecord::new(point, Vec3::default(), root, false);
        hit_record.set_face_and_normal(ray, (point - self.center) / self.radius);

        Some(hit_record)
    }
}
