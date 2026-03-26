#![feature(decl_macro)]
#![feature(portable_simd)]

pub mod vec;

mod binary_int;
pub use binary_int::BinaryInteger;

pub mod dist;

pub mod sphere;