use std::{
    ops::{Add, Div, Index, Mul, Sub},
    simd::cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd},
};

use crate::vec::{BVec3, TMask, TUnit, TVec};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Vec3<U> {
    pub x: U,
    pub y: U,
    pub z: U,
}
impl<U> Vec3<U> {
    #[inline]
    pub const fn new(x: U, y: U, z: U) -> Self {
        Vec3 { x, y, z }
    }
}
impl<U: Copy> Vec3<U> {
    #[inline]
    pub const fn splat(value: U) -> Self {
        Vec3::new(value, value, value)
    }
}
impl<U> Index<usize> for Vec3<U> {
    type Output = U;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}
impl<U: PartialEq> SimdPartialEq for Vec3<U> {
    type Mask = BVec3;

    fn simd_eq(self, other: Self) -> Self::Mask {
        BVec3::new(self.x == other.x, self.y == other.y, self.z == other.z)
    }

    fn simd_ne(self, other: Self) -> Self::Mask {
        BVec3::new(self.x != other.x, self.y != other.y, self.z != other.z)
    }
}
impl<U: PartialOrd> SimdPartialOrd for Vec3<U> {
    fn simd_lt(self, other: Self) -> Self::Mask {
        BVec3::new(self.x < other.x, self.y < other.y, self.z < other.z)
    }

    fn simd_le(self, other: Self) -> Self::Mask {
        BVec3::new(self.x <= other.x, self.y <= other.y, self.z <= other.z)
    }

    fn simd_gt(self, other: Self) -> Self::Mask {
        BVec3::new(self.x > other.x, self.y > other.y, self.z > other.z)
    }

    fn simd_ge(self, other: Self) -> Self::Mask {
        BVec3::new(self.x >= other.x, self.y >= other.y, self.z >= other.z)
    }
}
impl<U: TUnit> SimdOrd for Vec3<U> {
    fn simd_max(self, other: Self) -> Self {
        Vec3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    fn simd_min(self, other: Self) -> Self {
        Vec3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    fn simd_clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(min.simd_le(max).all());
        self.simd_max(min).simd_min(max)
    }
}
impl<U: TUnit> TVec<3> for Vec3<U> {
    type Unit = U;

    type Mask = BVec3;

    #[inline]
    fn from_array(array: [Self::Unit; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl<U: TUnit> Add for Vec3<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl<U: TUnit> Sub for Vec3<U> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl<U: TUnit> Mul for Vec3<U> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl<U: TUnit> Div for Vec3<U> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}
