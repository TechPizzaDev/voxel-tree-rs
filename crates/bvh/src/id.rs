use std::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct NodeId(NonZeroU32);
impl NodeId {
    pub const ZERO: Self = Self::new(0);

    #[inline]
    pub const fn new(value: u32) -> Self {
        NodeId(NonZeroU32::new(value + 1).unwrap())
    }
}
impl Default for NodeId {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
impl From<NodeId> for usize {
    #[inline]
    fn from(value: NodeId) -> Self {
        value.0.get() as usize - 1
    }
}
impl From<usize> for NodeId {
    #[inline]
    fn from(value: usize) -> Self {
        NodeId::new(value as u32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct LeafId(NonZeroU32);
impl LeafId {
    pub const ZERO: Self = Self::new(0);

    #[inline]
    pub const fn new(value: u32) -> Self {
        LeafId(NonZeroU32::new(value + 1).unwrap())
    }
}
impl Default for LeafId {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
impl From<LeafId> for usize {
    #[inline]
    fn from(value: LeafId) -> Self {
        value.0.get() as usize - 1
    }
}
impl From<usize> for LeafId {
    #[inline]
    fn from(value: usize) -> Self {
        LeafId::new(value as u32)
    }
}
