use glam::Vec3A;
use numerics::{dist::SqDist, sphere::Sphere};

use crate::node::NodeId;

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
