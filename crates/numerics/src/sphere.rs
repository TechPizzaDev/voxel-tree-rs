use glam::{Vec3A, Vec4};

use crate::vec::simd::vec3a_as_i32x4;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Sphere {
    xyzr: Vec4,
}
impl Sphere {
    #[inline]
    pub fn new(center: Vec3A, radius: f32) -> Self {
        Sphere {
            xyzr: center.extend(radius),
        }
    }

    #[inline]
    pub fn from_xyzr(xyzr: Vec4) -> Self {
        Sphere { xyzr }
    }

    #[inline]
    pub fn to_xyzr(self) -> Vec4 {
        self.xyzr
    }

    #[inline]
    pub fn center(&self) -> Vec3A {
        Vec3A::from_vec4(self.xyzr)
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.xyzr.w
    }

    #[inline]
    pub fn contains_point(&self, point: Vec3A) -> bool {
        self.intersects(&Self::new(point, 0.))
    }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        let center_dist_sq = self.center().distance_squared(other.center());
        let radius_sum_sq = self.radius() + other.radius();
        center_dist_sq <= (radius_sum_sq * radius_sum_sq)
    }
}

pub struct SphereVoxels {
    sphere: Sphere,
    start: Vec3A,
    end: Vec3A,
    curr: Vec4,
}
impl SphereVoxels {
    #[inline(never)]
    pub fn new(sphere: Sphere, step: f32) -> Self {
        let xyzr = sphere.to_xyzr() / step;
        let center = Vec3A::from_vec4(xyzr);
        let r = xyzr.w;

        let start = (center - r).floor();
        let end = (center + r).floor();
        SphereVoxels {
            sphere: Sphere::from_xyzr(xyzr),
            start,
            end,
            curr: start.extend(2.0),
        }
    }
}
impl Iterator for SphereVoxels {
    type Item = Vec3A;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.curr.z <= self.end.z {
            while self.curr.y <= self.end.y {
                while self.curr.x <= self.end.x {
                    let item = self.curr;
                    self.curr.x += 1.;

                    if self.sphere.intersects(&Sphere::from_xyzr(item)) {
                        return Some(Vec3A::from_vec4(item));
                    }
                }
                self.curr.x = self.start.x;
                self.curr.y += 1.;
            }
            self.curr.y = self.start.y;
            self.curr.z += 1.;
        }
        None
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SpatialKey<const R: usize> {
    x: i32,
    y: i32,
    z: i32,
}
impl<const R: usize> SpatialKey<R> {
    #[inline]
    pub fn from_grid(value: Vec3A) -> Self {
        let v = vec3a_as_i32x4(value);
        SpatialKey {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}
impl<const R: usize> From<Vec3A> for SpatialKey<R> {
    #[inline]
    fn from(value: Vec3A) -> Self {
        Self::from_grid((value / (R as f32)).floor())
    }
}

pub struct SphereSpatialKeys<const R: usize> {
    inner: SphereVoxels,
}
impl<const R: usize> From<Sphere> for SphereSpatialKeys<R> {
    fn from(value: Sphere) -> Self {
        SphereSpatialKeys {
            inner: SphereVoxels::new(value, R as f32),
        }
    }
}
impl<const R: usize> Iterator for SphereSpatialKeys<R> {
    type Item = SpatialKey<R>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(SpatialKey::from_grid)
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3A;

    use super::{Sphere, SphereSpatialKeys};

    // TODO: write proper tests

    #[test]
    fn sphere_keys() {
        const R: usize = 128;
        let keys: SphereSpatialKeys<R> = Sphere::new(Vec3A::new(0., 0., 0.), 4.).into();
        println!("{:?}", keys.collect::<Vec<_>>());
    }
}
