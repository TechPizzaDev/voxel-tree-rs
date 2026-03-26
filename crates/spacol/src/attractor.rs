use std::num::NonZero;

use glam::Vec3A;
use numerics::{dist::SqDist, sphere::Sphere};
use rstar::{AABB, PointDistance, RTreeObject};

use crate::{
    node::NodeId,
    rstar::{RPoint, RSphere},
};

#[derive(Clone)]
pub struct Attractor {
    pub point: Vec3A,
    pub influence: f32,

    node_dist: SqDist,
    node: Option<NodeId>,
}
impl Attractor {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::from(Vec3A::new(x, y, z))
    }

    #[inline]
    pub fn influence_sphere(&self) -> Sphere {
        Sphere::new(self.point, self.influence)
    }

    pub fn node(&self) -> Option<NodeId> {
        self.node
    }

    pub fn node_dist(&self) -> SqDist {
        self.node_dist
    }

    pub fn assign_node(&mut self, node: NodeId, dist: SqDist) {
        self.node_dist = dist;
        self.node = Some(node);
    }

    pub fn clear_node(&mut self) {
        self.node_dist = SqDist::INFINITY;
        self.node = None;
    }
}
impl From<Vec3A> for Attractor {
    #[inline]
    fn from(point: Vec3A) -> Self {
        Self {
            point,
            influence: 30.0,

            node_dist: SqDist::INFINITY,
            node: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AttrId(NonZero<u32>);
impl AttrId {
    pub const ZERO: AttrId = Self(NonZero::new(1).unwrap());

    #[inline]
    pub fn get(self) -> u32 {
        self.0.get().wrapping_sub(1)
    }
}
impl Default for AttrId {
    fn default() -> Self {
        Self::ZERO
    }
}
impl From<AttrId> for usize {
    #[inline]
    fn from(value: AttrId) -> Self {
        value.get() as usize
    }
}
impl TryFrom<u32> for AttrId {
    type Error = ();

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        value
            .checked_add(1)
            .and_then(NonZero::new)
            .map(AttrId)
            .ok_or(())
    }
}
impl TryFrom<usize> for AttrId {
    type Error = ();

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from(u32::try_from(value).map_err(|_| ())?)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AttrPoint {
    influence: RSphere,
    id: AttrId,
}
impl AttrPoint {
    #[inline]
    pub fn new(influence: RSphere, id: AttrId) -> Self {
        AttrPoint { influence, id }
    }

    #[inline]
    pub fn influence(&self) -> RSphere {
        self.influence
    }

    #[inline]
    pub fn id(&self) -> AttrId {
        self.id
    }
}
impl RTreeObject for AttrPoint {
    type Envelope = AABB<RPoint>;

    fn envelope(&self) -> Self::Envelope {
        self.influence.envelope()
    }
}
impl PointDistance for AttrPoint {
    fn distance_2(&self, point: &RPoint) -> f32 {
        self.influence.distance_2(point)
    }

    fn contains_point(&self, p: &RPoint) -> bool {
        self.influence.contains_point(p)
    }

    fn distance_2_if_less_or_equal(&self, point: &RPoint, max_distance_2: f32) -> Option<f32> {
        self.influence
            .distance_2_if_less_or_equal(point, max_distance_2)
    }
}
