use std::num::NonZero;

use glam::{Vec3, Vec3A};
use rstar::{AABB, PointDistance, RTreeObject};

use crate::rstar::RPoint;

#[derive(Clone)]
pub struct Node {
    pub point: Vec3A,
    pub parent: NodeId,

    // accumulation
    pub grow_dir: Vec3A,
    pub connected_attractors: u32,
}
impl Node {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::from(Vec3A::new(x, y, z))
    }

    #[inline]
    pub fn set_parent(&mut self, parent: NodeId) -> NodeId {
        std::mem::replace(&mut self.parent, parent)
    }
}
impl From<Vec3A> for Node {
    #[inline]
    fn from(point: Vec3A) -> Self {
        Self {
            point,
            parent: NodeId::ZERO,

            grow_dir: Vec3A::ZERO,
            connected_attractors: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct NodeId(NonZero<u32>);
impl NodeId {
    pub const ZERO: NodeId = Self(NonZero::new(1).unwrap());

    #[inline]
    pub fn get(self) -> u32 {
        self.0.get().wrapping_sub(1)
    }
}
impl Default for NodeId {
    fn default() -> Self {
        Self::ZERO
    }
}
impl From<NodeId> for usize {
    #[inline]
    fn from(value: NodeId) -> Self {
        value.get() as usize
    }
}
impl TryFrom<u32> for NodeId {
    type Error = ();

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        value
            .checked_add(1)
            .and_then(NonZero::new)
            .map(NodeId)
            .ok_or(())
    }
}
impl TryFrom<usize> for NodeId {
    type Error = ();

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from(u32::try_from(value).map_err(|_| ())?)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct NodePoint {
    point: Vec3,
    id: NodeId,
}
impl NodePoint {
    #[inline]
    pub fn new(point: Vec3, id: NodeId) -> Self {
        NodePoint { point, id }
    }

    #[inline]
    pub fn center(&self) -> Vec3A {
        let v: std::simd::f32x4 = unsafe { std::mem::transmute(*self) };
        Vec3A::from(v)
    }

    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }
}
impl RTreeObject for NodePoint {
    type Envelope = AABB<RPoint>;

    #[inline]
    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.center().into())
    }
}
impl PointDistance for NodePoint {
    #[inline]
    fn distance_2(&self, point: &RPoint) -> f32 {
        self.center().distance_squared(point.0)
    }

    #[inline]
    fn contains_point(&self, point: &RPoint) -> bool {
        self.center() == point.0
    }

    #[inline]
    fn distance_2_if_less_or_equal(&self, point: &RPoint, max_distance_2: f32) -> Option<f32> {
        let distance_2 = self.distance_2(point);
        if distance_2 <= max_distance_2 {
            return Some(distance_2);
        }
        None
    }
}
