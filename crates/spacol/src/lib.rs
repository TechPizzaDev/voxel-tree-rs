#![feature(portable_simd)]

mod algo;
pub use algo::{AttractorTreeParams, GrowError, KillFunction, NodeRecord, TreeMachine};

mod attractor;
pub use attractor::Attractor;

mod node;
pub use node::{Node, NodeId, NodePoint};

pub mod rstar;
