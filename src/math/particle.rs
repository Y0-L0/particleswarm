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

    pub fn update(&mut self) {
        self.velocity += &self.acceleration;
        self.position += &self.velocity;
    }
}

#[cfg(test)]
mod test {
    use core::f64;

    use approx::assert_relative_eq;
    use rand::{SeedableRng, rngs::StdRng};

    use crate::math::Vec3D;

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

    #[test]
    fn test_update() {
        let origin = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let velocity = origin.clone();
        let acceleration = Vec3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let mut particle = Particle {
            position: origin,
            velocity,
            acceleration: acceleration.clone(),
        };

        particle.update();

        assert_relative_eq!(
            acceleration.norm1(),
            particle.position.norm1(),
            epsilon = f64::EPSILON
        );

        particle.update();

        assert_relative_eq!(3.0 * 3.0, particle.position.norm1(), epsilon = f64::EPSILON);
        assert_relative_eq!(3.0 * 2.0, particle.velocity.norm1(), epsilon = f64::EPSILON);
    }
}
