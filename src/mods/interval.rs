#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// Default interval is empty.
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// The half-open ray range: from the camera plane outward to infinity.
    pub const NON_NEGATIVE: Self = Self {
        min: 0.0,
        max: f64::INFINITY,
    };

    #[must_use]
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    #[must_use]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    #[must_use]
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
