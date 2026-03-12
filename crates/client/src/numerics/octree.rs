use glam::Vec3A;
use oktree::prelude::TUVec3;

#[inline(always)]
pub fn vec3a_as_u32x4(v: Vec3A) -> [u32; 4] {
    if cfg!(target_feature = "avx512f") {
        use std::arch::x86_64::*;
        let v: __m128 = v.into();
        unsafe { core::mem::transmute(_mm_cvttps_epu32(v)) }
    } else {
        [v.x as u32, v.y as u32, v.z as u32, 0]
    }
}

#[inline(always)]
pub fn vec3a_as_tuvec3(v: Vec3A) -> TUVec3<u32> {
    let p = vec3a_as_u32x4(v);
    TUVec3::new(p[0], p[1], p[2])
}

#[inline(always)]
pub fn tuvec3_as_vec3a(v: TUVec3<u32>) -> Vec3A {
    if cfg!(target_feature = "fma") {
        use std::arch::x86_64::*;
        unsafe {
            let v0 = _mm_setr_epi32(v.x as i32, v.y as i32, v.z as i32, 0);
            let v1 = _mm_srli_epi32::<16>(v0);
            let v1 = _mm_cvtepi32_ps(v1);
            let v0 = _mm_and_si128(v0, _mm_set1_epi32(0xFFFF));
            let v0 = _mm_cvtepi32_ps(v0);
            let v1 = _mm_fmadd_ps(v1, _mm_castsi128_ps(_mm_set1_epi32(0x47800000)), v0);
            let v1 = v1.into();
            debug_assert_eq!(v1, Vec3A::from(v));
            v1
        }
    } else {
        v.into()
    }
}
