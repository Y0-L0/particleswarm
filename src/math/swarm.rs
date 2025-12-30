use crate::math::{Particle, Vec3D};

pub trait Swarm {
    fn update(&mut self);
    fn iter_positions(&self) -> impl Iterator<Item = &Vec3D>;
}

impl Swarm for Vec<Particle> {
    fn update(&mut self) {
        for p in self.iter_mut() {
            p.update();
        }
    }

    fn iter_positions(&self) -> impl Iterator<Item = &Vec3D> {
        return self.iter().map(|p| &p.position);
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3D;

    use super::*;
    use approx::assert_relative_eq;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn test_update() {
        let position = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let velocity = position.clone();
        let acceleration = Vec3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let particle1 = Particle {
            position: position.clone(),
            velocity: velocity.clone(),
            acceleration: acceleration.clone(),
        };
        let particle2 = Particle {
            position,
            velocity,
            acceleration: -acceleration.clone(),
        };
        let mut swarm = vec![particle1, particle2];

        swarm.update();

        assert_relative_eq!(
            acceleration.norm1(),
            swarm[0].position.norm1(),
            epsilon = f64::EPSILON
        );
        assert_relative_eq!(
            acceleration.norm1(),
            swarm[1].position.norm1(),
            epsilon = f64::EPSILON
        );
    }

    #[test]
    fn test_positions_iterator() {
        let mut rng = StdRng::seed_from_u64(42);
        let swarm = vec![Particle::spawn(&mut rng), Particle::spawn(&mut rng)];

        assert_eq!(
            2,
            swarm.iter_positions().count(),
            "Swarm of 2 particles must iterate two positions!"
        );
    }
}
