use crate::numerics::TMask;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BVec3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}
impl BVec3 {
    #[inline]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        BVec3 { x, y, z }
    }
}
impl TMask for BVec3 {
    #[inline]
    fn to_bitmask(self) -> u64 {
        (self.x as u64) | ((self.y as u64) << 1) | ((self.z as u64) << 1)
    }

    #[inline]
    fn all(self) -> bool {
        self.x && self.y && self.z
    }

    #[inline]
    fn any(self) -> bool {
        self.x || self.y || self.z
    }

    #[inline]
    fn none(self) -> bool {
        !self.x && !self.y && !self.z
    }
}
