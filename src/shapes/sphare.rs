use crate::{
    ray::Ray,
    shapes::hittable::{HitRecord, Hittable},
    vec3::{Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct Sphare {
    pub center: Point3,
    pub radius: f64,
}

impl Sphare {
    #[must_use]
    pub const fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphare {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;

        let mut rec = HitRecord {
            t: root,
            point: p,
            normal: Vec3::default(), // will be set below
            front_face: false,       // will be set below
        };
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
