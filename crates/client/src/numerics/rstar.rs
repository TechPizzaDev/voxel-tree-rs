use std::ops::{Deref, DerefMut};

use glam::Vec3A;
use rstar::AABB;

use crate::numerics::spatial::Sphere;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct RPoint(pub Vec3A);
impl rstar::Point for RPoint {
    type Scalar = f32;

    const DIMENSIONS: usize = 3;

    #[inline(always)]
    fn generate(mut generator: impl FnMut(usize) -> f32) -> Self {
        RPoint(Vec3A::new(generator(0), generator(1), generator(2)))
    }

    #[inline(always)]
    fn nth(&self, index: usize) -> f32 {
        self.0[index]
    }

    #[inline(always)]
    fn nth_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }
}
impl From<Vec3A> for RPoint {
    #[inline]
    fn from(value: Vec3A) -> Self {
        RPoint(value)
    }
}
impl Deref for RPoint {
    type Target = Vec3A;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RPoint {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct RSphere(pub Sphere);
impl rstar::RTreeObject for RSphere {
    type Envelope = AABB<RPoint>;

    fn envelope(&self) -> Self::Envelope {
        let r = Vec3A::splat(self.0.radius());
        let p1 = (self.0.center() - r).into();
        let p2 = (self.0.center() + r).into();
        AABB::from_corners(p1, p2)
    }
}
impl rstar::PointDistance for RSphere {
    fn distance_2(&self, point: &RPoint) -> f32 {
        let d = self.0.center() - point.0;
        // TODO: avoid sqrt?
        let dist_to_origin = d.length();
        let dist_to_ring = dist_to_origin - self.0.radius();
        let dist_to_circle = f32::max(0.0, dist_to_ring);
        dist_to_circle * dist_to_circle
    }

    fn contains_point(&self, point: &RPoint) -> bool {
        let d = self.0.center() - point.0;
        let dist_to_origin_2 = d.length_squared();
        let radius_2 = self.0.radius() * self.0.radius();
        dist_to_origin_2 <= radius_2
    }
}
impl From<Sphere> for RSphere {
    #[inline]
    fn from(value: Sphere) -> Self {
        RSphere(value)
    }
}
impl Deref for RSphere {
    type Target = Sphere;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RSphere {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}