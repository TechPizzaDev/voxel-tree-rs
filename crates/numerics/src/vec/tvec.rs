use std::ops::Index;

use crate::vec::TUnit;

pub trait TMask: Copy {
    fn to_bitmask(self) -> u64;

    fn all(self) -> bool;

    #[inline]
    fn any(self) -> bool {
        self.to_bitmask() != 0
    }

    #[inline]
    fn none(self) -> bool {
        !self.any()
    }
}

pub trait TVecSwizzle<const N: usize> {
    const INDEX: [usize; N];
}

pub trait TVec<const N: usize>: Copy + Index<usize, Output = Self::Unit> {
    type Unit: TUnit;

    type Mask: TMask;

    fn from_array(array: [Self::Unit; N]) -> Self;

    #[inline]
    fn swizzle<I: TVecSwizzle<N>>(vector: Self) -> Self {
        Self::from_array(I::INDEX.map(|i| vector[i]))
    }

    #[inline]
    fn concat_swizzle<I: TVecSwizzle<N>>(first: Self, second: Self) -> Self {
        Self::from_array(I::INDEX.map(|i| if i < N { first[i] } else { second[i - N] }))
    }
}

pub macro vec_swizzle {
    (
        $vector:expr, $index:expr $(,)?
    ) => {
        {
            struct Impl;
            impl TVecSwizzle<{$index.len()}> for Impl {
                const INDEX: [usize; {$index.len()}] = $index;
            }
            TVec::swizzle::<Impl>($vector)
        }
    },
    (
        $first:expr, $second:expr, $index:expr $(,)?
    ) => {
        {
            struct Impl;
            impl TVecSwizzle<{$index.len()}> for Impl {
                const INDEX: [usize; {$index.len()}] = $index;
            }
            TVec::concat_swizzle::<Impl>($first, $second)
        }
    }
}
