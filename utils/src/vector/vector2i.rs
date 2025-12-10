use std::ops::Sub;

#[derive(PartialEq, Eq, Hash, PartialOrd, Debug, Clone, Copy)]
pub struct Vector2i {
    pub x: i64,
    pub y: i64,
}

impl Vector2i {
    pub fn new(x: i64, y: i64) -> Self {
        return Self { x, y };
    }

    pub fn distance_to(&self, other: Vector2i) -> f64 {
        let distance = self.distance_to_squared(&other) as f64;
        distance.sqrt()
    }

    /// This is the distance squared.
    pub fn distance_to_squared(&self, other: &Vector2i) -> i64 {
        let difference = *self - *other;
        difference.x.pow(2) + difference.y.pow(2)
    }

    pub fn area(&self) -> i64 {
        (self.x.abs() + 1) * (self.y.abs() + 1)
    }
}

impl Sub for Vector2i {
    type Output = Vector2i;

    fn sub(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
