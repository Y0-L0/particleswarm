use crate::math::Vec3D;
use rand::distr::{Distribution, Uniform};
use rand::prelude::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Particle {
    pub position: Vec3D,
    pub velocity: Vec3D,
    pub acceleration: Vec3D,
}

impl Particle {
    pub fn spawn(rng: &mut impl Rng) -> Particle {
        // TODO: Fix unwrap.
        let between = Uniform::new(-1.0, 1.0).unwrap();
        let position = Vec3D {
            x: between.sample(rng),
            y: between.sample(rng),
            z: between.sample(rng),
        };
        let velocity = Vec3D {
            x: between.sample(rng),
            y: between.sample(rng),
            z: between.sample(rng),
        };
        let acceleration = Vec3D {
            x: between.sample(rng),
            y: between.sample(rng),
            z: between.sample(rng),
        };
        Particle {
            position,
            velocity,
            acceleration,
        }
    }
}
