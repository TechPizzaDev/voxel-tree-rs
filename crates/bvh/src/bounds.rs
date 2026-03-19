use std::simd::cmp::{SimdOrd, SimdPartialOrd};

use crate::numerics::{TMask, TUnit, Vec3, vec_swizzle};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Aabb<U: TUnit> {
    pub min: Vec3<U>,
    pub max: Vec3<U>,
}
impl<U: TUnit> Aabb<U> {
    pub fn from_min_max(min: Vec3<U>, max: Vec3<U>) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, position: Vec3<U>) -> bool {
        let le = self.min.simd_le(position);
        let gt = self.max.simd_gt(position);

        le.all() && gt.all()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let min = self.max.simd_min(other.max);
        let max = self.min.simd_max(other.min);

        min.simd_gt(max).all()
    }

    #[inline]
    pub fn center(&self) -> Vec3<U> {
        (self.min + self.max) / Vec3::splat(num::cast(2).unwrap())
    }

    #[inline]
    pub fn split(&self) -> [Self; 8] {
        let center = self.center();
        [
            Aabb::from_min_max(self.min, center),
            Aabb::from_min_max(
                vec_swizzle!(center, self.min, [0, 4, 5]),
                vec_swizzle!(self.max, center, [0, 4, 5]),
            ),
            Aabb::from_min_max(
                vec_swizzle!(self.min, center, [0, 4, 2]),
                vec_swizzle!(center, self.max, [0, 4, 2]),
            ),
            Aabb::from_min_max(
                vec_swizzle!(center, self.min, [0, 1, 5]),
                vec_swizzle!(self.max, center, [0, 1, 5]),
            ),
            Aabb::from_min_max(
                vec_swizzle!(self.min, center, [0, 1, 5]),
                vec_swizzle!(center, self.max, [0, 1, 5]),
            ),
            Aabb::from_min_max(
                vec_swizzle!(center, self.min, [0, 4, 2]),
                vec_swizzle!(self.max, center, [0, 4, 2]),
            ),
            Aabb::from_min_max(
                vec_swizzle!(self.min, center, [0, 4, 5]),
                vec_swizzle!(center, self.max, [0, 4, 5]),
            ),
            Aabb::from_min_max(center, self.max),
        ]
    }
}
