use std::num::{NonZero, NonZeroU32};

use bevy_math::bounding::{Aabb3d, BoundingSphere, IntersectsVolume};
use glam::Vec3A;
use oktree::{
    ElementId, Position,
    prelude::{Aabb, TUVec3},
    tree::Octree,
};
use rand::{Rng, distr::Distribution};
use tracing::trace_span;

use crate::{
    app::point_cloud::{Point, PointGen, Rgba},
    numerics::{dist::SqDist, octree::*},
};

pub type TreeId = u32;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct NodeId(NonZeroU32);
impl NodeId {
    #[inline]
    pub fn get(self) -> u32 {
        self.0.get().wrapping_sub(1)
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

#[derive(Clone)]
pub struct Attractor {
    point: Vec3A,
    node: Option<ElementId>,
}
impl Attractor {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::from(Vec3A::new(x, y, z))
    }
}
impl From<Vec3A> for Attractor {
    #[inline]
    fn from(point: Vec3A) -> Self {
        Self { point, node: None }
    }
}
impl Position for Attractor {
    type U = TreeId;

    #[inline]
    fn position(&self) -> TUVec3<Self::U> {
        vec3a_as_tuvec3(self.point)
    }
}

#[derive(Clone)]
pub struct Node {
    point: Vec3A,
    parent: Option<ElementId>,

    // accumulation
    grow_dir: Vec3A,
    connected_attractors: u32,
}
impl Node {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::from(Vec3A::new(x, y, z))
    }

    pub fn set_parent(&mut self, parent: Option<ElementId>) -> Option<ElementId> {
        std::mem::replace(&mut self.parent, parent)
    }
}
impl From<Vec3A> for Node {
    #[inline]
    fn from(point: Vec3A) -> Self {
        Self {
            point,
            parent: None,

            grow_dir: Vec3A::ZERO,
            connected_attractors: 0,
        }
    }
}
impl Position for Node {
    type U = TreeId;

    #[inline]
    fn position(&self) -> TUVec3<Self::U> {
        vec3a_as_tuvec3(self.point)
    }
}

#[derive(Debug)]
pub enum GrowError {
    Empty,
    OutOfReach,
    Limited,
}

#[derive(Clone, Debug)]
pub struct NodeRecord {
    pub start: u32,
    pub count: u32,
}

#[derive(Clone)]
pub struct TreeMachine {
    attractors: Octree<TreeId, Attractor>,
    nodes: Octree<TreeId, Node>,
    records: Vec<NodeRecord>,

    influence_radius: f32,
    kill_distance: f32,
    distance_factor: f32,
}
impl TreeMachine {
    /// At the beginning of tree generation, the space within the envelope
    /// is seeded with a set of attraction points (a). These points
    /// signal the availability of empty space for growth, and are
    /// removed when reached by a branch. The distribution of the
    /// attraction points is a user-controlled attribute of the method;
    /// some possibilities are outlined in Section 3.
    ///
    /// Given the attraction points, the tree skeleton is formed in an iterative process,
    /// beginning with a single node at the base of the tree (a).
    pub fn new(attractors: Octree<TreeId, Attractor>, nodes: Octree<TreeId, Node>) -> Self {
        Self {
            attractors,
            nodes,
            records: Vec::new(),

            influence_radius: 290.0,
            kill_distance: 2.6,
            distance_factor: 2.0,
        }
    }

    pub fn push_node(&mut self, node: Node) {
        self.nodes.insert(node).unwrap();
    }

    pub fn attractors(&self) -> &Octree<TreeId, Attractor> {
        &self.attractors
    }

    pub fn nodes(&self) -> &Octree<TreeId, Node> {
        &self.nodes
    }

    pub fn node_records(&self) -> &Vec<NodeRecord> {
        &self.records
    }

    /// In each iteration, new nodes, delimiting short branch segments,
    /// extend the skeleton in the direction of nearby attraction points (b, c).
    ///
    /// This process terminates when
    ///  * all attraction points have been removed,
    ///  * no nodes are within the radius of influence of the remaining attraction points,
    ///  * a user-specified number of iterations has been reached.
    pub fn grow(&mut self) -> Result<(), GrowError> {
        if self.attractors.is_empty() {
            return Err(GrowError::Empty);
        }

        let _grow = trace_span!("grow").entered();

        let mut node_buf = Vec::new();
        trace_span!("assign_nodes_to_attractors")
            .in_scope(|| self.assign_nodes_to_attractors(&mut node_buf));
        node_buf.clear();

        trace_span!("grow_nodes_toward_attractors")
            .in_scope(|| self.grow_nodes_toward_attractors(&mut node_buf));
        if node_buf.is_empty() {
            return Err(GrowError::OutOfReach);
        }

        let new_nodes =
            trace_span!("create_node_branches").in_scope(|| self.create_node_branches(&node_buf));

        trace_span!("kill_attractors").in_scope(|| self.kill_attractors(&new_nodes));

        Ok(())
    }

    fn assign_nodes_to_attractors(&mut self, node_buf: &mut Vec<ElementId>) {
        let max_dist = self.influence_radius;
        let sq_max_dist = SqDist::from_dist(max_dist);

        for a in self.attractors.iter_mut() {
            let influence_sphere = BoundingSphere::new(a.point, max_dist);

            self.nodes.extend_intersect_with(
                move |aabb| {
                    let aabb =
                        Aabb3d::from_min_max(tuvec3_as_vec3a(aabb.min), tuvec3_as_vec3a(aabb.max));
                    influence_sphere.intersects(&aabb)
                },
                node_buf,
            );

            // TODO: add config to pick random node from nearby attractors

            let mut sq_min_dist = SqDist::MAX;
            let mut min_node = None;
            for &node in node_buf.iter() {
                let np = self.nodes.get_element(node).unwrap();
                let sq_dist = SqDist::new(a.point.distance_squared(np.point));
                if sq_dist < sq_max_dist && sq_dist < sq_min_dist {
                    sq_min_dist = sq_dist;
                    min_node = Some(node);
                }
            }
            a.node = min_node;

            node_buf.clear();
        }
    }

    fn grow_nodes_toward_attractors(&mut self, connected_nodes: &mut Vec<ElementId>) {
        for n in self.nodes.iter_mut() {
            n.connected_attractors = 0;
        }

        for s in self.attractors.iter() {
            let Some(v_id) = s.node else {
                continue;
            };
            let v = self.nodes.get_element_mut(v_id).unwrap();
            if v.connected_attractors == 0 {
                connected_nodes.push(v_id);
            }

            v.grow_dir += (s.point - v.point).normalize();
            v.connected_attractors += 1;
        }
    }

    fn create_node_branches(&mut self, connected_nodes: &[ElementId]) -> Vec<ElementId> {
        let record_start = self.nodes.len();
        let mut new_nodes = Vec::new();

        let additional_nodes = connected_nodes.len();

        for &connected_node in connected_nodes {
            let n = self.nodes.get_element(connected_node).unwrap();
            debug_assert!(n.connected_attractors > 0);

            // TODO: attempt to not grow backwards into parent node?
            let dir = n.grow_dir.normalize();
            let mut node = Node::from(n.point + self.distance_factor * dir);
            node.set_parent(Some(connected_node));

            match self.nodes.insert(node) {
                Ok(id) => new_nodes.push(id),
                Err(err) => (), // TODO
            }
        }
        self.records.push(NodeRecord {
            start: u32::try_from(record_start).unwrap(),
            count: u32::try_from(additional_nodes).unwrap(),
        });

        new_nodes
    }

    fn kill_attractors(&mut self, nodes: &[ElementId]) {
        let d_k = self.kill_distance;
        let mut attractor_buf = Vec::new();

        for &node in nodes {
            let v = self.nodes.get_element(node).unwrap().point;
            let kill_sphere = BoundingSphere::new(v, d_k);

            self.attractors.extend_intersect_with(
                move |aabb| {
                    let aabb =
                        Aabb3d::from_min_max(tuvec3_as_vec3a(aabb.min), tuvec3_as_vec3a(aabb.max));
                    kill_sphere.intersects(&aabb)
                },
                &mut attractor_buf,
            );
            for &found in &attractor_buf {
                self.attractors.remove(found).unwrap();
            }
            attractor_buf.clear();
        }
    }

    /// The resulting tree skeleton may be further manipulated.
    /// First, the skeleton nodes may be decimated to
    /// reduce the amount of data representing the tree geometry (d).
    ///
    /// Moving each remaining node in parallel half way toward
    /// its more basal neighbor reduces the branching angles
    /// (compare the insets in Figures d and e)
    /// and can have a significant impact on the overall appearance of the tree.
    pub fn decimate(&mut self) {
        todo!()
    }

    /// Curve subdivision, extended to branching structures [[`PSSK03`]],
    /// can be applied to the original or decimated skeleton to create more smoothly curved limbs (f).
    ///
    /// [`PSSK03`]: https://doi.org/10.1142/S0218654303000048
    pub fn subdivide(&mut self) {
        todo!()
    }
}

pub struct SpaceColony {
    tree: TreeMachine,
}
impl SpaceColony {
    pub fn new_demo() -> Self {
        let points = [
            Vec3A::new(0.3, 3., 0.),
            Vec3A::new(-1.5, 1., 0.),
            Vec3A::new(2., -0.3, 0.),
            Vec3A::new(-2., -3., 0.),
        ];
        let mut attractors = Octree::with_capacity(4);
        for p in points {
            attractors.insert(Attractor::from(p)).unwrap();
        }

        let mut nodes = Octree::with_capacity(1);
        nodes.insert(Node::new(0., 0., 0.)).unwrap();

        let mut tree = TreeMachine::new(attractors, nodes);
        for i in 1..6 {
            tree.push_node(Node::new(0., -i as f32, 0.));
        }

        Self { tree }
    }

    pub fn with_rng(count: usize, rng: &mut impl Rng) -> Self {
        let x_distr = rand::distr::Uniform::new(0., 200.).unwrap();
        let y_distr = rand::distr::Uniform::new(0., 300.).unwrap();
        let z_distr = rand::distr::Uniform::new(0., 200.).unwrap();

        let aabb = Aabb::from_min_max(TUVec3::splat(0), TUVec3::splat(512));

        let mut attractors = Octree::from_aabb_with_capacity(aabb, count);
        for _i in 0..count {
            let x = x_distr.sample(rng);
            let y = y_distr.sample(rng);
            let z = z_distr.sample(rng);
            let a = Attractor::new(x, y, z);
            let key = vec3a_as_tuvec3(a.point);
            attractors.entry(key).or_insert(a);
        }

        let mut nodes = Octree::from_aabb_with_capacity(aabb, 1);
        nodes.insert(Node::new(0., 0., 0.)).unwrap();

        let tree = TreeMachine::new(attractors, nodes);
        Self { tree }
    }

    pub fn grow(&mut self) -> Result<(), GrowError> {
        self.tree.grow()
    }

    pub fn tree(&self) -> &TreeMachine {
        &self.tree
    }
}
impl PointGen for SpaceColony {
    fn generate(&self, output: &mut Vec<Point>) {
        for node in self.tree.nodes.iter() {
            output.push(Point {
                position: node.point.into(),
                color: Rgba::new(50, 50, 50, 127),
                size: 1.,
            });
        }

        for attractor in self.tree.attractors.iter() {
            output.push(Point {
                position: attractor.point.into(),
                color: Rgba::new(60, 180, 180, 127),
                size: 1.,
            });
        }
    }
}
