use crate::mods::{
    interval::Interval,
    ray::Ray,
    shapes::hittable::{HitRecord, Hittable, Shapes},
};

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
    fn hit(&self, ray: &Ray, ray_distance_interval: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_distance_interval.max;
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(temp_hit_record) = object.hit(
                ray,
                Interval::new(ray_distance_interval.min, closest_so_far),
            ) {
                closest_so_far = temp_hit_record.distance;
                hit_anything = Some(temp_hit_record);
            }
        }

        hit_anything
    }
}
