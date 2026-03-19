use crate::{
    LeafId, NodeId,
    bounds::Aabb,
    numerics::{TUnit, Vec3},
    pool::Pool,
};

#[derive(Clone, Copy, Debug)]
pub struct Node<U: TUnit> {
    pub aabb: Aabb<U>,
    pub ty: NodeType,
    pub parent: NodeId,
}

impl<U: TUnit> Node<U> {
    #[inline]
    pub fn from_aabb(aabb: Aabb<U>, parent: NodeId) -> Self {
        Node {
            aabb,
            parent,
            ty: NodeType::default(),
        }
    }

    #[inline(always)]
    pub(crate) fn branch(pool: &mut Pool<Self>, parent: NodeId) -> [NodeId; 8] {
        let aabbs = pool[parent.into()].aabb.split();
        core::array::from_fn(|i| pool.insert(Self::from_aabb(aabbs[i], parent)).into())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeType {
    Leaf(Option<LeafId>),
    Branch(Branch),
}
impl NodeType {
    pub const EMPTY: Self = Self::Leaf(None);
}
impl Default for NodeType {
    #[inline]
    fn default() -> Self {
        Self::EMPTY
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Branch {
    pub children: [NodeId; 8],
}
impl Branch {
    pub(crate) fn new(children: [NodeId; 8]) -> Self {
        Branch { children }
    }

    #[inline(always)]
    pub fn x0_y0_z0(&self) -> NodeId {
        self.children[0]
    }

    #[inline(always)]
    pub fn x1_y0_z0(&self) -> NodeId {
        self.children[1]
    }

    #[inline(always)]
    pub fn x0_y1_z0(&self) -> NodeId {
        self.children[2]
    }

    #[inline(always)]
    pub fn x1_y1_z0(&self) -> NodeId {
        self.children[3]
    }

    #[inline(always)]
    pub fn x0_y0_z1(&self) -> NodeId {
        self.children[4]
    }

    #[inline(always)]
    pub fn x1_y0_z1(&self) -> NodeId {
        self.children[5]
    }

    #[inline(always)]
    pub fn x0_y1_z1(&self) -> NodeId {
        self.children[6]
    }

    #[inline(always)]
    pub fn x1_y1_z1(&self) -> NodeId {
        self.children[7]
    }

    #[inline]
    pub fn center<U: TUnit>(&self, nodes: &Pool<Node<U>>) -> Vec3<U> {
        let node = &nodes[self.x0_y0_z0().into()];
        node.aabb.max
    }

    #[inline]
    pub(crate) fn walk_children_inclusive<U: TUnit>(
        &self,
        nodes: &Pool<Node<U>>,
        aabb: &Aabb<U>,
        mut f: impl FnMut(NodeId),
    ) {
        let center = self.center(nodes);
        if aabb.min.x <= center.x {
            if aabb.min.y <= center.y {
                if aabb.min.z <= center.z {
                    f(self.x0_y0_z0());
                }
                if aabb.max.z >= center.z {
                    f(self.x0_y0_z1());
                }
            }
            if aabb.max.y >= center.y {
                if aabb.min.z <= center.z {
                    f(self.x0_y1_z0());
                }
                if aabb.max.z >= center.z {
                    f(self.x0_y1_z1());
                }
            }
        }
        if aabb.max.x >= center.x {
            if aabb.min.y <= center.y {
                if aabb.min.z <= center.z {
                    f(self.x1_y0_z0());
                }
                if aabb.max.z >= center.z {
                    f(self.x1_y0_z1());
                }
            }
            if aabb.max.y >= center.y {
                if aabb.min.z <= center.z {
                    f(self.x1_y1_z0());
                }
                if aabb.max.z >= center.z {
                    f(self.x1_y1_z1());
                }
            }
        }
    }

    #[inline]
    pub(crate) fn walk_children_exclusive<U: TUnit>(
        &self,
        nodes: &Pool<Node<U>>,
        aabb: &Aabb<U>,
        mut f: impl FnMut(NodeId),
    ) {
        let center = self.center(nodes);
        if aabb.min.x < center.x {
            if aabb.min.y < center.y {
                if aabb.min.z < center.z {
                    f(self.x0_y0_z0());
                }
                if aabb.max.z > center.z {
                    f(self.x0_y0_z1());
                }
            }
            if aabb.max.y > center.y {
                if aabb.min.z < center.z {
                    f(self.x0_y1_z0());
                }
                if aabb.max.z > center.z {
                    f(self.x0_y1_z1());
                }
            }
        }
        if aabb.max.x > center.x {
            if aabb.min.y < center.y {
                if aabb.min.z < center.z {
                    f(self.x1_y0_z0());
                }
                if aabb.max.z > center.z {
                    f(self.x1_y0_z1());
                }
            }
            if aabb.max.y > center.y {
                if aabb.min.z < center.z {
                    f(self.x1_y1_z0());
                }
                if aabb.max.z > center.z {
                    f(self.x1_y1_z1());
                }
            }
        }
    }

    /// Search which octant is suitable for the position.
    ///
    /// * `position`: Element's position
    /// * `center`: center of the current node's [`Aabb`]
    #[inline(always)]
    pub fn find_child<U: TUnit>(&self, position: Vec3<U>, center: Vec3<U>) -> NodeId {
        let x = if position.x < center.x { 0 } else { 1 };
        let y = if position.y < center.y { 0 } else { 1 };
        let z = if position.z < center.z { 0 } else { 1 };

        let idx = x | (y << 1) | (z << 2);
        self.children[idx]
    }
}
