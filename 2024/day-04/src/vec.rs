use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Vec2 {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Vec2 {
    pub(crate) const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub(crate) fn scale(&self, factor: i32) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
