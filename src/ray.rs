use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
