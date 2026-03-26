use std::simd::{f32x4, num::SimdFloat};

use glam::Vec3A;

use crate::vec::Vec3;

#[inline(always)]
pub fn vec3a_as_u32x4(v: Vec3A) -> [u32; 4] {
    f32x4::from(v).cast().to_array()
}

#[inline(always)]
pub fn vec3a_as_uvec3(v: Vec3A) -> Vec3<u32> {
    let p = vec3a_as_u32x4(v);
    Vec3::new(p[0], p[1], p[2])
}

#[inline(always)]
pub fn vec3a_as_i32x4(v: Vec3A) -> [i32; 4] {
    f32x4::from(v).cast().to_array()
}
