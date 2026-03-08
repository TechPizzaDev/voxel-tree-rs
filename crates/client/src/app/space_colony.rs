use std::num::{NonZero, NonZeroU32};

use glam::Vec3A;
use rand::{Rng, distr::Distribution};

use crate::{
    app::point_cloud::{Point, PointGen, Rgba},
    numerics::dist::SqDist,
};

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct NodeIndex(NonZeroU32);
impl NodeIndex {
    #[inline]
    pub fn get(self) -> u32 {
        self.0.get().wrapping_sub(1)
    }
}
impl From<NodeIndex> for usize {
    #[inline]
    fn from(value: NodeIndex) -> Self {
        value.get() as usize
    }
}
impl TryFrom<u32> for NodeIndex {
    type Error = ();

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        value
            .checked_add(1)
            .and_then(NonZero::new)
            .map(NodeIndex)
            .ok_or(())
    }
}
impl TryFrom<usize> for NodeIndex {
    type Error = ();

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from(u32::try_from(value).map_err(|_| ())?)
    }
}

#[derive(Clone)]
pub struct Attractor {
    point: Vec3A,
    node: Option<NodeIndex>,
}
impl Attractor {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            point: Vec3A::new(x, y, z),
            node: None,
        }
    }
}

#[derive(Clone)]
pub struct Node {
    point: Vec3A,
    parent: Option<NodeIndex>,

    // accumulation
    grow_dir: Vec3A,
    connected_attractors: u32,
}
impl Node {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            point: Vec3A::new(x, y, z),
            parent: None,

            grow_dir: Vec3A::ZERO,
            connected_attractors: 0,
        }
    }

    pub fn set_parent(&mut self, parent: Option<NodeIndex>) -> Option<NodeIndex> {
        std::mem::replace(&mut self.parent, parent)
    }
}
impl From<Vec3A> for Node {
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

#[derive(Clone)]
pub struct TreeMachine {
    attractors: Vec<Attractor>,
    nodes: Vec<Node>,

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
    pub fn new(attractors: Vec<Attractor>, root: Node) -> Self {
        Self {
            attractors,
            nodes: vec![root],

            influence_radius: 20.0,
            kill_distance: 2.6,
            distance_factor: 2.0,
        }
    }

    pub fn push_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn attractors(&self) -> &Vec<Attractor> {
        &self.attractors
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
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

        let max_dist = SqDist::from_dist(self.influence_radius);
        for a in &mut self.attractors {
            a.node = Self::closest_node(a.point.into(), max_dist, self.nodes.iter()).0;
        }
        for n in &mut self.nodes {
            n.connected_attractors = 0;
        }

        let mut additional_nodes = 0usize;
        for s in &self.attractors {
            let Some(v) = s.node else {
                continue;
            };
            let v = &mut self.nodes[usize::from(v)];
            if v.connected_attractors == 0 {
                additional_nodes += 1;
            }

            v.grow_dir += (s.point - v.point).normalize();
            v.connected_attractors += 1;
        }
        if additional_nodes == 0 {
            return Err(GrowError::OutOfReach);
        }

        self.nodes.reserve(additional_nodes);
        for i in 0..self.nodes.len() {
            let n = &self.nodes[i];
            if n.connected_attractors <= 0 {
                continue;
            }

            let dir = n.grow_dir.normalize();
            let mut node = Node::from(n.point + self.distance_factor * dir);
            node.set_parent(Some(NodeIndex::try_from(i).unwrap()));
            self.nodes.push(node);
        }

        let nodes = &self.nodes;
        let d_k = SqDist::from_dist(self.kill_distance);
        self.attractors.retain(move |s| {
            let s = s.point;
            for v in nodes {
                if SqDist::new(s.distance_squared(v.point)) < d_k {
                    return false;
                }
            }
            true
        });

        Ok(())
    }

    fn closest_node<'a>(
        point: Vec3A,
        max_dist: SqDist,
        nodes: impl Iterator<Item = &'a Node>,
    ) -> (Option<NodeIndex>, SqDist) {
        let mut min_dist = SqDist::new(f32::MAX);
        let mut min_node = None;
        for (i, node) in nodes.enumerate() {
            let dist = SqDist::new(point.distance_squared(node.point.into()));
            if dist < max_dist && dist < min_dist {
                min_dist = dist;
                min_node = Some(NodeIndex::try_from(i).unwrap());
            }
        }
        (min_node, min_dist)
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
        let attractors = vec![
            Attractor::new(0.3, 3., 0.),
            Attractor::new(-1.5, 1., 0.),
            Attractor::new(2., -0.3, 0.),
            Attractor::new(-2., -3., 0.),
        ];

        let mut tree = TreeMachine::new(attractors, Node::new(0., 0., 0.));
        for i in 1..6 {
            tree.push_node(Node::new(0., -i as f32, 0.));
        }

        Self { tree }
    }

    pub fn with_rng(count: usize, rng: &mut impl Rng) -> Self {
        let x_distr = rand::distr::Uniform::new(-100., 100.).unwrap();
        let y_distr = rand::distr::Uniform::new(-100., 200.).unwrap();
        let z_distr = rand::distr::Uniform::new(-100., 100.).unwrap();

        let mut attractors = Vec::with_capacity(count);
        for i in 0..count {
            let x = x_distr.sample(rng);
            let y = y_distr.sample(rng);
            let z = z_distr.sample(rng);
            attractors.push(Attractor::new(x, y, z));
        }

        let tree = TreeMachine::new(attractors, Node::new(0., -100., 0.));
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
        for node in &self.tree.nodes {
            output.push(Point {
                position: node.point.into(),
                color: Rgba::rgb(50, 50, 50),
                size: 1.,
            });
        }

        for attractor in &self.tree.attractors {
            output.push(Point {
                position: attractor.point.into(),
                color: Rgba::rgb(60, 180, 180),
                size: 1.,
            });
        }
    }
}
