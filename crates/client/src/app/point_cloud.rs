use bytemuck::{Pod, Zeroable};
use glam::Vec3;

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Rgba {
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct Point {
    pub position: Vec3,
    pub size: f32,
    pub color: Rgba,
}

pub trait PointGen {
    fn generate(&self, output: &mut Vec<Point>);
}

pub struct PointSphere {
    pub count: i32,
    pub radius: f32,
    pub point_size: f32,
    pub point_color: Rgba,
}
impl PointGen for PointSphere {
    fn generate(&self, output: &mut Vec<Point>) {
        let increment = std::f32::consts::PI * (3. - 5f32.sqrt());
        let radius = 1.;
        let offset = 2. / (self.count as f32);
        for i in 0..self.count {
            let y = ((i as f32 * offset) - 1.) + (offset / 2.);
            let r = (1. - (y * y)).sqrt();
            let phi = i as f32 * increment;
            let (sin, cos) = phi.sin_cos();
            let x = cos * r;
            let z = sin * r;
            output.push(Point {
                position: Vec3::new(x, y, z) * radius,
                size: self.point_size,
                color: self.point_color
            });
        }
    }
}
