use std::{
    num::{NonZero, NonZeroU32},
    time::{Duration, Instant},
};

use bvh::{VolId, Volume, VolumeHash, ZeroHasher};
use bytemuck::Zeroable;
use glam::Vec3A;
use hashbrown::HashSet;
use rand::{Rng, distr::Distribution};
use tracing::trace_span;

use crate::{
    app::point_cloud::{Point, PointGen, Rgba},
    numerics::{
        dist::SqDist,
        spatial::{SpatialKey, Sphere, SphereSpatialKeys},
    },
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
        Sphere::new(self.point, self.influence).keys()
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

pub struct TreeMachine {
    attractors: VolumeHash<SpatialKey<A_R>, Attractor>,
    nodes: Vec<Node>,
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
    pub fn new(attractors: VolumeHash<SpatialKey<A_R>, Attractor>, nodes: Vec<Node>) -> Self {
        Self {
            attractors,
            nodes,
            records: Vec::new(),

            kill_distance: 9.6,
            distance_factor: 2.0,
        }
    }

    pub fn push_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn attractors(&self) -> &VolumeHash<SpatialKey<A_R>, Attractor> {
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
        let kill_count = span.in_scope(|| self.kill_attractors(&new_nodes));
        span.record("count", kill_count);
        drop(span);

        Ok(())
    }

    fn assign_nodes_to_attractors(&mut self) {
        let mut a_set: HashSet<VolId, ZeroHasher> = HashSet::default();

        for node_id in 0..self.nodes.len() {
            let node = &self.nodes[node_id];
            let node_id = NodeId::try_from(node_id).unwrap();

            // find potential attractors
            for a_bucket in self
                .attractors
                .buckets_in_volume(&Sphere::new(node.point, 0.))
            {
                a_set.extend(a_bucket.items());
            }

            // TODO: add config to pick random node from nearby attractors

            for a in a_set.drain() {
                let Some(a) = self.attractors.get_mut(a) else {
                    continue;
                };
                let dist = SqDist::new(Vec3A::distance_squared(a.point, node.point));
                if dist <= a.node_dist {
                    a.node_dist = dist;
                    a.node = Some(node_id);
                }
            }
        }
    }

    fn grow_nodes_toward_attractors(&mut self, connected_nodes: &mut Vec<NodeId>) {
        // TODO: only iterate attractors near nodes (based on attr:node ratio)?

        for s in self.attractors.values_mut() {
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

    fn create_node_branches(&mut self, connected_nodes: &[NodeId]) -> Vec<NodeId> {
        let record_start = self.nodes.len();
        let mut new_nodes = Vec::with_capacity(connected_nodes.len());

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

            let id = NodeId::try_from(self.nodes.len()).unwrap();
            self.nodes.push(child);
            new_nodes.push(id);
        }
        self.records.push(NodeRecord {
            start: u32::try_from(record_start).unwrap(),
            count: u32::try_from(additional_nodes).unwrap(),
        });

        new_nodes
    }

    fn kill_attractors(&mut self, nodes: &[NodeId]) -> usize {
        let d_k = self.kill_distance;

        let mut a_set: HashSet<VolId, ZeroHasher> = HashSet::default();
        let mut counter = 0;

        for &node in nodes {
            let v = self.nodes[usize::from(node)].point;
            let kill_sphere = Sphere::new(v, d_k);

            // find potential attractors
            a_set.extend(self.attractors.items_in_volume(&kill_sphere));

            for a_id in a_set.drain() {
                if let Some(a) = self.attractors.get(a_id) {
                    if kill_sphere.contains_point(a.point) {
                        self.attractors.remove(a_id).unwrap();
                        counter += 1;
                    }
                }
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
        let mut attractors: VolumeHash<SpatialKey<A_R>, Attractor> = VolumeHash::new();
        for p in points {
            attractors.insert(Attractor::from(p));
        }

        let mut nodes = Vec::with_capacity(1);
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

        let mut attractors = VolumeHash::new();
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

        println!(
            "construction time: \n  sample: {:?},\n  insert: {:?}",
            sample_time, insert_time
        );

        let mut nodes = Vec::with_capacity(1);
        nodes.push(Node::new(0., 0., 0.));

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

        for attractor in self.tree.attractors.values() {
            output.push(Point {
                position: attractor.point.into(),
                color: Rgba::new(60, 180, 180, 127),
                size: 1.,
            });
        }
    }
}
