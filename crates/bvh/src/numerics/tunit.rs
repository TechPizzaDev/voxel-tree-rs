use std::ops::{Add, Div, Mul, Sub};

use num::NumCast;

pub trait NativeOrd: Sized + PartialOrd {
    #[inline]
    fn max(self, other: Self) -> Self {
        if other < self { self } else { other }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        if other < self { other } else { self }
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(min < max);
        self.max(min).min(max)
    }
}

pub trait TUnit:
    Copy
    + Default
    + NumCast
    + NativeOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
{
}

impl TUnit for f32 {}
impl NativeOrd for f32 {
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }

    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }
}

impl TUnit for u32 {}
impl NativeOrd for u32 {}

impl TUnit for i32 {}
impl NativeOrd for i32 {}
