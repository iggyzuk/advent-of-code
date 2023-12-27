use std::fmt::Display;

/// Vector 2
///
/// A vector with positive right and down components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Clone + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vec2<T>
where
    T: std::ops::Neg<Output = T>,
    T: Clone + Copy,
{
    pub fn rotate_left(&mut self) {
        // ( 1,  0)
        // ( 0, -1)
        // (-1, -0)
        // (-0,  1)
        std::mem::swap(&mut self.x, &mut self.y);
        self.y = -self.y;
    }

    pub fn rotate_right(&mut self) {
        // ( 1,  0)
        // (-0,  1)
        // (-1,  0)
        // ( 0, -1)
        std::mem::swap(&mut self.x, &mut self.y);
        self.x = -self.x;
    }
}

impl Vec2<isize> {
    pub const ZERO: Vec2<isize> = Vec2 { x: 0, y: 0 };
    pub const ONE: Vec2<isize> = Vec2 { x: 1, y: 1 };

    pub const LEFT: Vec2<isize> = Vec2 { x: -1, y: 0 };
    pub const RIGHT: Vec2<isize> = Vec2 { x: 1, y: 0 };
    pub const UP: Vec2<isize> = Vec2 { x: 0, y: -1 };
    pub const DOWN: Vec2<isize> = Vec2 { x: 0, y: 1 };
}

impl<T> std::ops::Add for Vec2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Vec2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> std::ops::Sub for Vec2<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Vec2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> std::ops::Mul for Vec2<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> std::ops::Neg for Vec2<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2() {
        let v = Vec2::new(1, 2);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn vec2_left_rot() {
        let mut v = Vec2::new(1, 0);
        v.rotate_left();
        assert_eq!(v, Vec2::new(0, -1));
        v.rotate_left();
        assert_eq!(v, Vec2::new(-1, 0));
        v.rotate_left();
        assert_eq!(v, Vec2::new(0, 1));
        v.rotate_left();
        assert_eq!(v, Vec2::new(1, 0));
    }

    #[test]
    fn vec2_right_rot() {
        let mut v = Vec2::new(1, 0);
        v.rotate_right();
        assert_eq!(v, Vec2::new(0, 1));
        v.rotate_right();
        assert_eq!(v, Vec2::new(-1, 0));
        v.rotate_right();
        assert_eq!(v, Vec2::new(0, -1));
        v.rotate_right();
        assert_eq!(v, Vec2::new(1, 0));
    }
}
