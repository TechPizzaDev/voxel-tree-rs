
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SqDist(pub f32);

impl SqDist {
    pub fn new(d_squared: f32) -> Self {
        Self(d_squared)
    }

    pub fn from_dist(d: f32) -> Self {
        Self(d * d)
    }

    pub fn to_dist(self) -> f32 {
        self.0.sqrt()
    }
}