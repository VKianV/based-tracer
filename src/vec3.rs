use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

pub type RGB = Vec3;
pub type Point3 = Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub const fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn x(self) -> f64 {
        self.0
    }

    pub fn y(self) -> f64 {
        self.1
    }

    pub fn z(self) -> f64 {
        self.2
    }

    pub fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self::new(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
