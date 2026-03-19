use smallvec::SmallVec;

use crate::{
    Aabb, InsertError, LeafId, NodeId,
    node::{Branch, Node, NodeType},
    numerics::TUnit,
    pool::Pool,
};

pub trait Volume<U: TUnit> {
    fn volume(&self) -> Aabb<U>;
}

pub struct Octree<T, U: TUnit = f32> {
    pub(crate) root: NodeId,

    pub(crate) nodes: Pool<Node<U>>,

    pub(crate) leaves: Pool<T>,
}
impl<T: Volume<U>, U: TUnit> Octree<T, U> {
    pub fn insert(&mut self, item: T) -> Result<LeafId, InsertError<T>> {
        let volume = item.volume();
        if !self.nodes[self.root.into()].aabb.intersects(&volume) {
            return Err(InsertError::OutOfBounds(item));
        }
        let leaf: LeafId = self.leaves.reserve().into();

        let mut insertions: SmallVec<[Insertion<U>; 10]> = SmallVec::new();
        insertions.push(Insertion {
            leaf,
            node: self.root,
            volume,
        });

        let mut inserted = false;
        while let Some(insertion) = insertions.pop() {
            inserted |= self._insert(insertion, &mut insertions) == Some(leaf);
        }

        if !inserted {
            self.leaves.remove(leaf.into());
            return Err(InsertError::Occupied(item));
        }

        self.leaves.replace(leaf.into(), item);
        Ok(leaf)
    }

    #[inline]
    fn _insert<const C: usize>(
        &mut self,
        insertion: Insertion<U>,
        insertions: &mut SmallVec<[Insertion<U>; C]>,
    ) -> Option<LeafId> {
        let Insertion {
            leaf: element,
            node,
            volume,
        } = insertion;

        let n = &mut self.nodes[node.into()];
        match n.ty {
            NodeType::Leaf(None) => {
                n.ty = NodeType::Leaf(Some(element));
                Some(element)
            }

            NodeType::Leaf(Some(e)) => {
                if n.aabb.unit() {
                    return None; // ignore
                }

                let e1 = self.leaves[e.into()].volume();
                let e2 = self.leaves[element.into()].volume();
                if e1.intersects(&e2) {
                    return None;
                }

                let children = Node::branch(&mut self.nodes, node);
                let n = &mut self.nodes[node.into()];

                n.ty = NodeType::Branch(Branch::new(children));
                insertions.push(insertion);
                insertions.push(Insertion {
                    leaf: e,
                    node,
                    volume: e1,
                });
                None
            }

            NodeType::Branch(branch) => {
                branch.walk_children_exclusive(&self.nodes, &volume, |child| {
                    insertions.push(Insertion {
                        leaf: element,
                        node: child,
                        volume,
                    });
                });
                None
            }
        }
    }
}

#[derive(Debug)]
struct Insertion<U: TUnit> {
    leaf: LeafId,
    node: NodeId,
    volume: Aabb<U>,
}

#[derive(Debug)]
struct Removal {
    parent: Option<NodeId>,
    node: NodeId,
}
