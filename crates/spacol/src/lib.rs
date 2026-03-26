#![feature(portable_simd)]

mod algo;
pub use algo::{AttractorTreeParams, GrowError, NodeRecord, SpaCol};

mod attractor;
pub use attractor::{AttrId, AttrPoint, Attractor};

mod node;
pub use node::{Node, NodeId, NodePoint};

pub mod rstar;
