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

    #[inline]
    fn generate(mut generator: impl FnMut(usize) -> f32) -> Self {
        Self(Vec3A::new(generator(0), generator(1), generator(2)))
    }

    #[inline(always)]
    fn nth(&self, index: usize) -> f32 {
        self.0[index]
    }

    #[inline(always)]
    fn nth_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }

    #[inline(always)]
    fn new() -> Self {
        Self(Vec3A::ZERO)
    }

    #[inline]
    fn le_point_all(&self, other: &Self) -> bool {
        self.0.cmple(other.0).all()
    }

    #[inline]
    fn ge_point_all(&self, other: &Self) -> bool {
        self.0.cmpge(other.0).all()
    }

    #[inline]
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.0.dot(rhs.0)
    }

    #[inline]
    fn reduce_sum(&self) -> Self::Scalar {
        self.0.element_sum()
    }

    #[inline]
    fn reduce_product(&self) -> Self::Scalar {
        self.0.element_product()
    }

    #[inline]
    fn splat(value: Self::Scalar) -> Self {
        Self(Vec3A::splat(value))
    }

    #[inline]
    fn min_point(&self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }

    #[inline]
    fn max_point(&self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }

    #[inline]
    fn length_2(&self) -> Self::Scalar {
        self.0.length_squared()
    }

    #[inline]
    fn sub(&self, other: &Self) -> Self {
        Self(self.0 - other.0)
    }

    #[inline]
    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0)
    }

    #[inline]
    fn mul(&self, other: &Self) -> Self {
        Self(self.0 * other.0)
    }

    #[inline]
    fn div(&self, other: &Self) -> Self {
        Self(self.0 / other.0)
    }

    #[inline]
    fn distance_2(&self, other: &Self) -> Self::Scalar {
        self.0.distance_squared(other.0)
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
        let radius = self.radius();
        radius * radius
    }
}
impl rstar::RTreeObject for RSphere {
    type Envelope = AABB<RPoint>;

    #[inline]
    fn envelope(&self) -> Self::Envelope {
        let c = self.center();
        let r = Vec3A::splat(self.radius());
        let lower = (c - r).into();
        let upper = (c + r).into();
        AABB::from_corners_unchecked(lower, upper)
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

    #[inline]
    fn distance_2_if_less_or_equal(&self, point: &RPoint, max_distance_2: f32) -> Option<f32> {
        // Sphere distance is cheap, so avoid envelope check,
        // since the envelope box always encloses the sphere.
        let distance_2 = self.distance_2(point);
        if distance_2 <= max_distance_2 {
            return Some(distance_2);
        }
        None
    }
}
impl From<Sphere> for RSphere {
    #[inline]
    fn from(value: Sphere) -> Self {
        RSphere(value)
    }
}
