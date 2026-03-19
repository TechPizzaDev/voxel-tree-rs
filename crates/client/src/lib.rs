#![feature(portable_simd)]

mod app;
pub use app::{App, FrameIndex};

mod egui_tools;

mod platform;

mod sync;

mod gpu;

mod numerics;