use crate::{b2Vec2, b2Rot, b2Transform};

impl std::ops::Add for b2Vec2 {
    type Output = Self;

    fn add(self, rhs: b2Vec2) -> b2Vec2 {
        b2Vec2{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for b2Vec2 {
    type Output = Self;

    fn sub(self, rhs: b2Vec2) -> b2Vec2 {
        b2Vec2{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for b2Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> b2Vec2 {
        b2Vec2{
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl b2Vec2 {
    pub fn zero() -> Self {
        Self {x: 0.0, y: 0.0}
    }
}

impl b2Transform {
    pub fn identity() -> Self {
        Self {
            p: b2Vec2::zero(),
            q: b2Rot::identity()
        }
    }
}

impl b2Rot {
    pub fn identity() -> Self {
        Self {s: 0.0, c: 1.0}
    }

    pub fn from_angle(x: f32) -> Self {
        Self {s: x.sin(), c: x.cos()}
    }

    pub fn angle(&self) -> f32 {
        self.s.atan2(self.c)
    }
}
