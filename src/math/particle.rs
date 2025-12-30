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

    fn update(&mut self) {
        // TODO cont
        // self.velocity += self.acceleration;
    }
}

#[cfg(test)]
mod test {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn spawn() {
        let mut rng = StdRng::seed_from_u64(1);
        let particle = Particle::spawn(&mut rng);

        assert_ne!(0.0, particle.acceleration.x);
        assert_ne!(0.0, particle.acceleration.y);
        assert_ne!(0.0, particle.acceleration.z);
        assert_ne!(0.0, particle.position.x);
        assert_ne!(0.0, particle.position.y);
        assert_ne!(0.0, particle.position.z);
        assert_ne!(0.0, particle.velocity.x);
        assert_ne!(0.0, particle.velocity.y);
        assert_ne!(0.0, particle.velocity.z);
    }
}
