use core::f64;

use crate::math::{Particle, Vec3D};

pub trait Swarm {
    fn update(&mut self);
    fn iter_positions(&self) -> impl Iterator<Item = &Vec3D>;
    fn closest_index(&self, to_what: &Vec3D) -> Option<usize>;
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

    fn closest_index(&self, to_what: &Vec3D) -> Option<usize> {
        let mut min_index = None;
        let mut min_norm = f64::INFINITY;
        for (idx, norm) in self
            .iter_positions()
            .map(|p| (p - &to_what).norm1())
            .enumerate()
        {
            if norm < min_norm {
                min_index = Some(idx);
                min_norm = norm;
            }
        }
        return min_index;
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

    #[test]
    fn test_closest_idx() {
        let position1 = Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let position2 = Vec3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let disturbance = Vec3D {
            x: 0.1,
            y: -0.1,
            z: 0.11,
        };

        let particle1 = Particle {
            position: position1.clone(),
            velocity: Vec3D::zero(),
            acceleration: Vec3D::zero(),
        };
        let particle2 = Particle {
            position: position2.clone(),
            velocity: Vec3D::zero(),
            acceleration: Vec3D::zero(),
        };

        let swarm = vec![particle1, particle2];

        let close1 = swarm
            .closest_index(&(&position1 + &disturbance))
            .expect("No closest found around particle 1");
        assert_eq!(0, close1);

        let close2 = swarm
            .closest_index(&(&position2 + &disturbance))
            .expect("No closest found around particle 2");
        assert_eq!(1, close2);
    }
}
