use crate::{
    ray::Ray,
    shapes::sphare::Sphare,
    vec3::{Point3, Vec3},
};

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the hit record normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub enum Shapes {
    Sphare(Sphare),
}

impl Hittable for Shapes {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        match self {
            Self::Sphare(s) => s.hit(ray, ray_tmin, ray_tmax),
            // Object::Plane(p) => p.hit(...),
        }
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Shapes>,
}

impl HittableList {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_object(object: Shapes) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Shapes) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }
}
