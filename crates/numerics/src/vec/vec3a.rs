use std::simd::{Mask, MaskElement};

impl<T: MaskElement, const N: usize> TMask for Mask<T, N> {
    fn to_bitmask(self) -> u64 {
        Mask::to_bitmask(self)
    }

    fn all(self) -> bool {
        Mask::all(self)
    }

    fn any(self) -> bool {
        Mask::any(self)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3A<U: SimdElement>(Simd<U, 4>);
impl<U: SimdElement> Index<usize> for Vec3A<U> {
    type Output = U;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<U: SimdElement + PartialEq> PartialEq for Vec3A<U> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<U: SimdElement> Add for Vec3A<U>
where
    Simd<U, 4>: Add<Output = Simd<U, 4>>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(Simd::add(self.0, rhs.0))
    }
}
impl<U: SimdElement> Sub for Vec3A<U>
where
    Simd<U, 4>: Sub<Output = Simd<U, 4>>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(Simd::sub(self.0, rhs.0))
    }
}
impl<U: SimdElement> Mul for Vec3A<U>
where
    Simd<U, 4>: Mul<Output = Simd<U, 4>>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(Simd::mul(self.0, rhs.0))
    }
}
impl<U: SimdElement> Div for Vec3A<U>
where
    Simd<U, 4>: Div<Output = Simd<U, 4>>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(Simd::div(self.0, rhs.0))
    }
}

impl<U: SimdElement> SimdPartialEq for Vec3A<U>
where
    Simd<U, 4>: SimdPartialEq,
    <Simd<U, 4> as SimdPartialEq>::Mask: TMask,
{
    type Mask = <Simd<U, 4> as SimdPartialEq>::Mask;

    fn simd_eq(self, other: Self) -> Self::Mask {
        Simd::simd_eq(self.0, other.0)
    }

    fn simd_ne(self, other: Self) -> Self::Mask {
        Simd::simd_ne(self.0, other.0)
    }
}
impl<U: SimdElement> SimdPartialOrd for Vec3A<U>
where
    Simd<U, 4>: SimdPartialOrd,
    <Simd<U, 4> as SimdPartialEq>::Mask: TMask,
{
    fn simd_lt(self, other: Self) -> Self::Mask {
        Simd::simd_lt(self.0, other.0)
    }

    fn simd_le(self, other: Self) -> Self::Mask {
        Simd::simd_le(self.0, other.0)
    }

    fn simd_gt(self, other: Self) -> Self::Mask {
        Simd::simd_gt(self.0, other.0)
    }

    fn simd_ge(self, other: Self) -> Self::Mask {
        Simd::simd_ge(self.0, other.0)
    }
}
impl<U: SimdElement> SimdOrd for Vec3A<U>
where
    Simd<U, 4>: SimdOrd,
    <Simd<U, 4> as SimdPartialEq>::Mask: TMask,
{
    fn simd_max(self, other: Self) -> Self {
        Self(Simd::simd_max(self.0, other.0))
    }

    fn simd_min(self, other: Self) -> Self {
        Self(Simd::simd_min(self.0, other.0))
    }

    fn simd_clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(min.simd_le(max).all());
        self.simd_max(min).simd_min(max)
    }
}

impl<U: SimdElement + TUnit> TVec<3> for Vec3A<U>
where
    Simd<U, 4>: SimdOrd,
    <Simd<U, 4> as SimdPartialEq>::Mask: TMask,
{
    type Unit = U;

    type Mask = <Simd<U, 4> as SimdPartialEq>::Mask;

    fn from_array(array: [Self::Unit; 3]) -> Self {
        Self(Simd::from_array([
            array[0],
            array[1],
            array[2],
            U::default(),
        ]))
    }

    fn swizzle<I: VecSwizzle<3>>(a: Self) -> Self {
        use core::simd::Swizzle;

        struct Impl<I>(PhantomData<I>);
        impl<I: VecSwizzle<3>> Swizzle<4> for Impl<I> {
            const INDEX: [usize; 4] = const { [I::INDEX[0], I::INDEX[1], I::INDEX[2], 3] };
        }
        Self(Impl::<I>::swizzle(a.0))
    }

    fn concat_swizzle<I: VecSwizzle<3>>(first: Self, second: Self) -> Self {
        use core::simd::Swizzle;

        struct Impl<I>(PhantomData<I>);
        impl<I: VecSwizzle<3>> Swizzle<4> for Impl<I> {
            const INDEX: [usize; 4] = const { [I::INDEX[0], I::INDEX[1], I::INDEX[2], 3] };
        }
        Self(Impl::<I>::concat_swizzle(first.0, second.0))
    }
}
