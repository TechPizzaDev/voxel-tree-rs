
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct SqDist(pub f32);

impl SqDist {
    pub const MAX: Self = Self(f32::MAX);
    pub const INFINITY: Self = Self(f32::INFINITY);

    pub const fn new(d_squared: f32) -> Self {
        Self(d_squared)
    }

    pub const fn from_dist(d: f32) -> Self {
        Self(d * d)
    }

    pub fn to_dist(self) -> f32 {
        self.0.sqrt()
    }
}