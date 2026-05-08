use std::time::{Duration, Instant};

use glam::{Vec3, Vec3A};
use pool::Pool;
use rand::{Rng, distr::Distribution};
use spacol::{Attractor, GrowError, Node, NodeId, SpaCol};

use crate::app::point_cloud::{Point, PointGen, Rgba};

pub struct SpaceColony {
    tree: SpaCol,
}
impl SpaceColony {
    pub fn new_demo() -> Self {
        let points = [
            Vec3A::new(0.3, 3., 0.),
            Vec3A::new(-1.5, 1., 0.),
            Vec3A::new(2., -0.3, 0.),
            Vec3A::new(-2., -3., 0.),
        ];
        let mut attractors = Pool::default();
        for p in points {
            attractors.insert(Attractor::from(p));
        }

        let mut nodes = Vec::new();
        nodes.push(Node::new(0., 0., 0.));

        let mut tree = SpaCol::new(attractors, nodes);
        for i in 1..6 {
            tree.push_node(Node::new(0., -i as f32, 0.));
        }

        Self { tree }
    }

    fn spherical(rng: &mut impl Rng) -> Vec3 {
        let u: f32 = rand::distr::StandardUniform.sample(rng);
        let v: f32 = rand::distr::StandardUniform.sample(rng);
        let theta = u * 2.0 * std::f32::consts::PI;
        let phi = (2.0 * v - 1.0).acos();
        let r = f32::cbrt(rand::distr::StandardUniform.sample(rng));
        let (sin_theta, cos_theta) = theta.sin_cos();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let x = r * sin_phi * cos_theta;
        let y = r * sin_phi * sin_theta;
        let z = r * cos_phi;
        return Vec3::new(x, y, z);
    }

    pub fn with_rng(count: usize, rng: &mut impl Rng) -> Self {
        let w = 200.;
        let h = 300.;
        let d = 200.;
        let scale = Vec3::new(w, h, d);

        let x_distr = rand::distr::Uniform::new(0., w).unwrap();
        let y_distr = rand::distr::Uniform::new(h * 0.25, h).unwrap();
        let z_distr = rand::distr::Uniform::new(0., d).unwrap();

        let mut sample_time = Duration::ZERO;
        let mut insert_time = Duration::ZERO;

        let mut attractors = Pool::default();
        for _i in 0..count {
            let start = Instant::now();
            let x = x_distr.sample(rng);
            let y = y_distr.sample(rng);
            let z = z_distr.sample(rng);

            let mid = Instant::now();

            let sphere_point = (Self::spherical(rng) + Vec3::splat(1.)) * 0.5;
            let Vec3 { x, y, z } =
                sphere_point * Vec3::new(w, h - h * 0.25, d) + Vec3::new(0., h * 0.25, 0.);

            let a = Attractor::new(x, y, z);
            //let key = vec3a_as_uvec3(a.point);
            attractors.insert(a);

            let end = Instant::now();

            sample_time += mid.duration_since(start);
            insert_time += end.duration_since(mid);
        }

        let mut nodes = Vec::new();
        nodes.push(Node::new(w / 2., 0., d / 2.));

        let start = Instant::now();
        let tree = SpaCol::new(attractors, nodes);
        let load_time = Instant::now().duration_since(start);

        println!(
            "construction time: \n  sample: {:?},\n  insert: {:?},\n  load: {:?}",
            sample_time, insert_time, load_time
        );

        Self { tree }
    }

    pub fn grow(&mut self) -> Result<(), GrowError> {
        self.tree.grow()
    }

    pub fn tree(&self) -> &SpaCol {
        &self.tree
    }
}
impl PointGen for SpaceColony {
    fn generate(&self, output: &mut Vec<Point>) {
        for (i, node) in self.tree.nodes().iter().enumerate() {
            let id = NodeId::try_from(i).unwrap();
            output.push(Point {
                position: node.point.into(),
                color: Rgba::new(50, 50, 50, 127),
                size: 1.,
            });
        }

        for attractor in self.tree.attractors().iter() {
            output.push(Point {
                position: attractor.point().into(),
                color: Rgba::new(60, 180, 180, 127),
                size: 1.,
            });
        }
    }
}
