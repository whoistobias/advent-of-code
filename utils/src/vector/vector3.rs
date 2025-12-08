use std::ops::Sub;

#[derive(PartialEq, Eq, Hash, PartialOrd, Debug, Clone, Copy)]
pub struct Vector3i {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector3i {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        return Self { x, y, z };
    }

    pub fn distance_to(&self, other: Vector3i) -> f64 {
        let distance = self.distance_to_squared(&other) as f64;
        distance.sqrt()
    }

    /// This is the distance squared.
    pub fn distance_to_squared(&self, other: &Vector3i) -> i64 {
        let difference = *self - *other;
        difference.x.pow(2) + difference.y.pow(2) + difference.z.pow(2)
    }
}

impl Sub for Vector3i {
    type Output = Vector3i;

    fn sub(self, rhs: Vector3i) -> Vector3i {
        Vector3i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
