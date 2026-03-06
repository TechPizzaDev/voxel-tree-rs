use glam::vec3;

use crate::app::point_cloud::{Point, PointGen, Rgba};

pub struct SpaceColony {}
impl PointGen for SpaceColony {
    fn generate(&self, output: &mut Vec<Point>) {
        output.push(Point {
            position: vec3(0., 0., 0.),
            size: 100.,
            color: Rgba::MAGENTA,
        });

        output.push(Point {
            position: vec3(0., 1., 0.),
            size: 100.,
            color: Rgba::AQUA,
        });
    }
}
