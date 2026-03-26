
pub trait BinaryInteger {
    fn round_up_to_pow_of_2(self) -> Self;

    fn containing_pow_of_2(self) -> u32;
}

impl BinaryInteger for u32 {
    #[inline]
    fn round_up_to_pow_of_2(self) -> Self {
        let shift = 32 - (self - 1).leading_zeros();
        return (1u32 ^ (shift >> 5)) << shift;
    }

    /// <summary>
    /// Computes the lowest integer N such that 2^N >= i.
    /// </summary>
    /// <param name="i">Integer to compute the power of.</param>
    /// <returns>Lowest integer N such that 2^N >= i.</returns>
    #[inline]
    fn containing_pow_of_2(self) -> u32 {
        let unsigned = if self == 0 { 1 } else { self };
        32 - (unsigned - 1).leading_zeros()
    }
}
impl BinaryInteger for u64 {
    #[inline]
    fn round_up_to_pow_of_2(self) -> Self {
        let shift = 64 - (self - 1).leading_zeros();
        return (1u64 ^ (shift as u64 >> 6)) << shift;
    }

    #[inline]
    fn containing_pow_of_2(self) -> u32 {
        let unsigned = if self == 0 { 1 } else { self };
        64 - (unsigned - 1).leading_zeros()
    }
}
impl BinaryInteger for usize {
    #[inline]
    fn round_up_to_pow_of_2(self) -> Self {
        match size_of::<usize>() {
            4 => (self as u32).round_up_to_pow_of_2() as usize,
            8 => (self as u64).round_up_to_pow_of_2() as usize,
            size => unimplemented!("{}", size),
        }
    }

    #[inline]
    fn containing_pow_of_2(self) -> u32 {
        match size_of::<usize>() {
            4 => (self as u32).containing_pow_of_2(),
            8 => (self as u64).containing_pow_of_2(),
            size => unimplemented!("{}", size),
        }
    }
}
