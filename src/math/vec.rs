
#[derive(Debug, Clone, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    fn norm1(&self) -> f64 {
        return self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm1_int() {
        let vec = Vec3D{x: 0.0, y: 0.0, z: 0.0};
        assert_eq!(0.0, vec.norm1())
    }
}
