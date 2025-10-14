use crate::b2Vec2;

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
