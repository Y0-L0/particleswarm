#[derive(Debug, Clone, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    fn norm1(&self) -> f64 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
}

#[cfg(test)]
mod tests {
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
}
