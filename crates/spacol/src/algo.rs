use glam::Vec3A;
use numerics::{dist::SqDist, sphere::Sphere};

use pool::Pool;
use rstar::{AABB, RTree};

use tracing::trace_span;

use crate::{AttrId, AttrPoint, Attractor, Node, NodeId, NodePoint, rstar::RPoint};

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
    attractor_tree: RTree<AttrPoint, AttractorTreeParams>,

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
                    AttrPoint::new(attr.influence_sphere().into(), AttrId::try_from(i).unwrap())
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
            let point = a.point();
            let influence_2 = a.influence() * a.influence();

            // TODO: add config to pick random node from nearby attractors

            if let Some((node, dist_2)) = self
                .node_tree
                .nearest_neighbor_in_range(point.into(), influence_2)
            {
                debug_assert_eq!(dist_2, node.center().distance_squared(point));
                debug_assert!(dist_2 <= influence_2);
                a.assign_node(node.id(), SqDist::from_dist(dist_2));
            }
        }
    }

    fn grow_nodes_toward_attractors(&mut self, connected_nodes: &mut Vec<NodeId>) {
        // TODO: only iterate attractors near nodes (based on attr:node ratio)?

        for s in self.attractors.iter_mut() {
            let Some(v_id) = s.node() else {
                continue;
            };
            let v = &mut self.nodes[usize::from(v_id)];
            if v.connected_attractors == 0 {
                connected_nodes.push(v_id);
            }

            v.grow_dir += (s.point() - v.point).normalize();
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
            child.set_parent(connected_node);

            // reset parent
            n.grow_dir = Vec3A::default();
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

            for attr in self
                .attractor_tree
                .drain_with_selection_function(KillFunction {
                    sphere: kill_sphere,
                })
            {
                let a = self.attractors.remove(attr.id().into()).unwrap();
                let dist = a.point().distance_squared(v);
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

struct KillFunction {
    sphere: Sphere,
}
impl rstar::SelectionFunction<AttrPoint> for KillFunction {
    fn should_unpack_parent(&self, parent_envelope: &AABB<RPoint>) -> bool {
        let envelope_dist_2 = parent_envelope.distance_2(&self.sphere.center().into());
        let r = self.sphere.radius();
        envelope_dist_2 <= (r * r)
    }

    fn should_unpack_leaf(&self, leaf: &AttrPoint) -> bool {
        self.sphere.contains_point(leaf.influence().center())
    }
}
