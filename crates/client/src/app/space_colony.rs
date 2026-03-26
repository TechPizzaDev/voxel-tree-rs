use std::{
    num::{NonZero, NonZeroU32},
    time::{Duration, Instant},
};

use bvh::{Pool, Volume};
use bytemuck::Zeroable;
use glam::{Vec3, Vec3A};
use rand::{Rng, distr::Distribution};
use rstar::{AABB, PointDistance, RTree, RTreeObject, primitives::GeomWithData};
use tracing::trace_span;

use crate::{
    app::point_cloud::{Point, PointGen, Rgba},
    numerics::rstar::{RPoint, RSphere},
};
use numerics::{
    dist::SqDist,
    sphere::{SpatialKey, Sphere, SphereSpatialKeys},
};

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
    influence: f32,

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
}
impl From<Vec3A> for Attractor {
    #[inline]
    fn from(point: Vec3A) -> Self {
        Self {
            point,
            influence: 30.0,

            node_dist: SqDist::MAX,
            node: None,
        }
    }
}
impl Volume for Attractor {
    type Key = SpatialKey<A_R>;

    type Iter = SphereSpatialKeys<A_R>;

    fn keys(&self) -> Self::Iter {
        self.influence_sphere().keys()
    }
}

const A_R: usize = 32;

impl Volume for Sphere {
    type Key = SpatialKey<A_R>;

    type Iter = SphereSpatialKeys<A_R>;

    #[inline]
    fn keys(&self) -> Self::Iter {
        self.clone().into()
    }
}

#[derive(Clone)]
pub struct Node {
    point: Vec3A,
    parent: Option<NodeId>,

    // accumulation
    grow_dir: Vec3A,
    connected_attractors: u32,
}
impl Node {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::from(Vec3A::new(x, y, z))
    }

    #[inline]
    pub fn set_parent(&mut self, parent: Option<NodeId>) -> Option<NodeId> {
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
impl NodeRecord {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = NodeId> {
        (self.start..(self.start + self.count)).map(|i| NodeId::try_from(i).unwrap())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct AttractorTreeParams;

impl rstar::RTreeParams for AttractorTreeParams {
    const MIN_SIZE: usize = 32;
    const MAX_SIZE: usize = 128;
    const REINSERTION_COUNT: usize = 16;
    type DefaultInsertionStrategy = rstar::RStarInsertionStrategy;
}

pub struct TreeMachine {
    attractors: Pool<Attractor>,
    attractor_tree: RTree<GeomWithData<RSphere, u32>, AttractorTreeParams>,

    nodes: Vec<Node>,
    node_tree: RTree<NodePoint>,
    records: Vec<NodeRecord>,

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
    pub fn new(attractors: Pool<Attractor>, nodes: Vec<Node>) -> Self {
        let node_tree = RTree::bulk_load(
            nodes
                .iter()
                .enumerate()
                .map(|(i, node)| NodePoint::new(node.point.into(), NodeId::try_from(i).unwrap()))
                .collect(),
        );

        let attractor_tree = RTree::bulk_load_with_params(
            attractors
                .iter()
                .enumerate()
                .map(|(i, attr)| {
                    GeomWithData::new(attr.influence_sphere().into(), u32::try_from(i).unwrap())
                })
                .collect(),
        );

        Self {
            attractors,
            attractor_tree,

            nodes,
            node_tree,
            records: Vec::new(),

            kill_distance: 9.6,
            distance_factor: 2.0,
        }
    }

    pub fn push_node(&mut self, node: Node) -> NodeId {
        let id = NodeId::try_from(self.nodes.len()).unwrap();
        let point = node.point.into();
        self.nodes.push(node);
        self.node_tree.insert(NodePoint::new(point, id));
        id
    }

    pub fn attractors(&self) -> &Pool<Attractor> {
        &self.attractors
    }

    pub fn nodes(&self) -> &Vec<Node> {
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
        use tracing::field::Empty;

        if self.attractors.is_empty() {
            return Err(GrowError::Empty);
        }

        let _grow = trace_span!("grow").entered();

        let mut node_buf = Vec::new();
        trace_span!("assign_nodes_to_attractors").in_scope(|| self.assign_nodes_to_attractors());
        node_buf.clear();

        let span = trace_span!("grow_nodes_toward_attractors", count = Empty);
        span.in_scope(|| {
            self.grow_nodes_toward_attractors(&mut node_buf);
        });
        span.record("count", node_buf.len());
        drop(span);

        if node_buf.is_empty() {
            return Err(GrowError::OutOfReach);
        }

        let new_nodes =
            trace_span!("create_node_branches").in_scope(|| self.create_node_branches(&node_buf));

        let span = trace_span!("kill_attractors", count = Empty);
        let kill_count = span.in_scope(|| self.kill_attractors(new_nodes));
        span.record("count", kill_count);
        drop(span);

        Ok(())
    }

    fn assign_nodes_to_attractors(&mut self) {
        for a in self.attractors.iter_mut() {
            let influence_2 = a.influence * a.influence;

            // TODO: add config to pick random node from nearby attractors

            if let Some((geo, dist_2)) = self
                .node_tree
                .nearest_neighbor_in_range(a.point.into(), influence_2)
            {
                debug_assert_eq!(dist_2, geo.center().distance_squared(a.point));
                debug_assert!(dist_2 <= influence_2);
                a.node = Some(geo.id());
            }
        }
    }

    fn grow_nodes_toward_attractors(&mut self, connected_nodes: &mut Vec<NodeId>) {
        // TODO: only iterate attractors near nodes (based on attr:node ratio)?

        for s in self.attractors.iter_mut() {
            let Some(v_id) = s.node else {
                continue;
            };
            let v = &mut self.nodes[usize::from(v_id)];
            if v.connected_attractors == 0 {
                connected_nodes.push(v_id);
            }

            v.grow_dir += (s.point - v.point).normalize();
            v.connected_attractors += 1;
        }
    }

    fn create_node_branches(&mut self, connected_nodes: &[NodeId]) -> NodeRecord {
        let record_start = self.nodes.len();

        let additional_nodes = connected_nodes.len();

        for &connected_node in connected_nodes {
            let n = &mut self.nodes[usize::from(connected_node)];
            debug_assert!(n.connected_attractors > 0);

            // TODO: attempt to not grow backwards into parent node?
            let dir = n.grow_dir.normalize();
            let mut child = Node::from(n.point + self.distance_factor * dir);
            child.set_parent(Some(connected_node));

            // reset parent
            n.grow_dir = Vec3A::zeroed();
            n.connected_attractors = 0;

            self.push_node(child);
        }

        let record = NodeRecord {
            start: u32::try_from(record_start).unwrap(),
            count: u32::try_from(additional_nodes).unwrap(),
        };
        self.records.push(record.clone());
        record
    }

    fn kill_attractors(&mut self, nodes: NodeRecord) -> usize {
        let d_k = self.kill_distance;

        let mut counter = 0;

        for node in nodes.iter() {
            let v = self.nodes[usize::from(node)].point;
            let kill_sphere = Sphere::new(v, d_k);

            for geo in self
                .attractor_tree
                .drain_with_selection_function(KillFunction {
                    sphere: kill_sphere,
                })
            {
                let a = self.attractors.remove(geo.data as usize).unwrap();
                let dist = a.point.distance_squared(v);
                debug_assert!(dist <= (d_k * d_k), "dist = {}, d_k = {}", dist, d_k * d_k);
                counter += 1;
            }
        }
        counter
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

pub struct KillFunction {
    sphere: Sphere,
}
impl rstar::SelectionFunction<GeomWithData<RSphere, u32>> for KillFunction {
    fn should_unpack_parent(&self, parent_envelope: &AABB<RPoint>) -> bool {
        let envelope_dist_2 = parent_envelope.distance_2(&self.sphere.center().into());
        let r = self.sphere.radius();
        envelope_dist_2 <= (r * r)
    }

    fn should_unpack_leaf(&self, leaf: &GeomWithData<RSphere, u32>) -> bool {
        self.sphere.contains_point(leaf.geom().0.center())
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
        let mut attractors = Pool::default();
        for p in points {
            attractors.insert(Attractor::from(p));
        }

        let mut nodes = Vec::new();
        nodes.push(Node::new(0., 0., 0.));

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

        let mut sample_time = Duration::ZERO;
        let mut insert_time = Duration::ZERO;

        let mut attractors = Pool::default();
        for _i in 0..count {
            let start = Instant::now();
            let x = x_distr.sample(rng);
            let y = y_distr.sample(rng);
            let z = z_distr.sample(rng);

            let mid = Instant::now();

            let a = Attractor::new(x, y, z);
            //let key = vec3a_as_uvec3(a.point);
            attractors.insert(a);

            let end = Instant::now();

            sample_time += mid.duration_since(start);
            insert_time += end.duration_since(mid);
        }

        let mut nodes = Vec::new();
        nodes.push(Node::new(0., 0., 0.));

        let start = Instant::now();
        let tree = TreeMachine::new(attractors, nodes);
        let load_time = Instant::now().duration_since(start);

        println!(
            "construction time: \n  sample: {:?},\n  insert: {:?},\n  load: {:?}",
            sample_time, insert_time, load_time
        );

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
