#![feature(portable_simd)]
#![feature(decl_macro)]

extern crate alloc;

mod bounds;
pub use bounds::Aabb;

mod id;
pub use id::{LeafId, NodeId};

pub mod numerics;

mod pool;

mod volume_hash;
pub use volume_hash::{Bucket, VolId, Volume, VolumeHash, ZeroHasher};

#[derive(Debug, PartialEq)]
pub enum InsertError<T> {
    /// Item is out of bounds of tree's [`Aabb`].
    OutOfBounds(T),

    /// Leaf is already occupied by an item.
    Occupied(T),
}

#[derive(Debug, PartialEq)]
pub enum RemoveError {
    /// Item is out of bounds of tree's [`Aabb`].
    OutOfBounds,

    /// Leaf is not found in the tree.
    NotFound,
}
