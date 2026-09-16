use crate::{
    ray::Ray,
    shapes::sphere::Sphere,
    vec3::{Point3, Vec3},
};

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
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub enum Shapes {
    Sphare(Sphere),
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
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(temp_hit_record) = object.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = temp_hit_record.distance;
                hit_anything = Some(temp_hit_record);
            }
        }

        hit_anything
    }
}
