use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn norm1(&self) -> f64 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
}

impl ops::Add<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<&Vec3D> for Vec3D {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Neg for Vec3D {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        return self;
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use super::*;
    use approx::assert_relative_eq;
    use test_case::test_case;

    #[test_case(0.0, 0.0, 0.0, 0.0 ; "zero vector")]
    #[test_case(0.2, 1.8, 0.0, 2.0 ; "positive vector")]
    #[test_case(2.0, -3.0, 0.0, 5.0 ; "negative vector")]
    #[test_case(0.0, -3.75, 0.2, 3.95 ; "fraction vector")]
    fn test_norm1_int(x: f64, y: f64, z: f64, expected: f64) {
        let vec = Vec3D { x, y, z };
        assert_relative_eq!(expected, vec.norm1(), epsilon = f64::EPSILON);
    }

    #[test]
    fn test_add_vec3d() {
        let vec1 = Vec3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec2 = Vec3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let vec_sum = vec1 + vec2;
        assert_relative_eq!(5.0, vec_sum.x, epsilon = f64::EPSILON);
        assert_relative_eq!(7.0, vec_sum.y, epsilon = f64::EPSILON);
        assert_relative_eq!(9.0, vec_sum.z, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_add_assign_vec3d() {
        let mut vec1 = Vec3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec2 = Vec3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        vec1 += &vec2;
        assert_relative_eq!(5.0, vec1.x, epsilon = f64::EPSILON);
        assert_relative_eq!(7.0, vec1.y, epsilon = f64::EPSILON);
        assert_relative_eq!(9.0, vec1.z, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_neg_vec3d() {
        let mut vec = Vec3D {
            x: 1.0,
            y: 2.0,
            z: -3.0,
        };
        vec = -vec;
        assert_relative_eq!(-1.0, vec.x, epsilon = f64::EPSILON);
        assert_relative_eq!(-2.0, vec.y, epsilon = f64::EPSILON);
        assert_relative_eq!(3.0, vec.z, epsilon = f64::EPSILON);
    }
}
