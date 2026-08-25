use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[must_use]
    pub const fn origin(&self) -> Point3 {
        self.origin
    }

    #[must_use]
    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
