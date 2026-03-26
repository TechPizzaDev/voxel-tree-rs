use std::ops::{Deref, DerefMut};

use glam::Vec3A;
use rstar::AABB;

use numerics::sphere::Sphere;

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
pub struct RSphere(Sphere);
impl RSphere {
    #[inline]
    pub fn center(&self) -> Vec3A {
        self.0.center()
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.0.radius()
    }

    #[inline]
    pub fn radius_2(&self) -> f32 {
        self.radius() * self.radius()
    }
}
impl rstar::RTreeObject for RSphere {
    type Envelope = AABB<RPoint>;

    #[inline]
    fn envelope(&self) -> Self::Envelope {
        let c = self.center();
        let r = Vec3A::splat(self.radius());
        let p1 = (c - r).into();
        let p2 = (c + r).into();
        AABB::from_corners(p1, p2)
    }
}
impl rstar::PointDistance for RSphere {
    #[inline]
    fn distance_2(&self, point: &RPoint) -> f32 {
        let d = self.center() - point.0;
        let dist_to_origin = d.length_squared();
        let dist_to_ring = dist_to_origin - self.radius_2();
        let dist_to_circle = f32::max(0.0, dist_to_ring);
        dist_to_circle
    }

    #[inline]
    fn contains_point(&self, point: &RPoint) -> bool {
        let d = self.center() - point.0;
        let dist_to_origin_2 = d.length_squared();
        dist_to_origin_2 <= self.radius_2()
    }
}
impl From<Sphere> for RSphere {
    #[inline]
    fn from(value: Sphere) -> Self {
        RSphere(value)
    }
}
