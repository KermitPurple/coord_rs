use num_traits::Num;
use std::ops::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Coord<T: Num + Copy> {
    x: T,
    y: T,
}

impl<T: Num + Copy> Coord<T> {
    pub fn new(x: T, y: T) -> Self{
        Self{x, y}
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> Add<I> for Coord<T> {
    type Output = Self;
    fn add(self, other: I) -> Self {
        let other = other.into();
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> AddAssign<I> for Coord<T> {
    fn add_assign(&mut self, other: I) {
        *self = *self + other;
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> Sub<I> for Coord<T> {
    type Output = Self;
    fn sub(self, other: I) -> Self {
        let other = other.into();
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> SubAssign<I> for Coord<T> {
    fn sub_assign(&mut self, other: I) {
        *self = *self - other;
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> Mul<I> for Coord<T> {
    type Output = Self;
    fn mul(self, other: I) -> Self {
        let other = other.into();
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> MulAssign<I> for Coord<T> {
    fn mul_assign(&mut self, other: I) {
        *self = *self * other;
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> Div<I> for Coord<T> {
    type Output = Self;
    fn div(self, other: I) -> Self {
        let other = other.into();
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> DivAssign<I> for Coord<T> {
    fn div_assign(&mut self, other: I) {
        *self = *self / other;
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> Rem<I> for Coord<T> {
    type Output = Self;
    fn rem(self, other: I) -> Self {
        let other = other.into();
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl<T: Num + Copy, I: Into<Coord<T>>> RemAssign<I> for Coord<T> {
    fn rem_assign(&mut self, other: I) {
        *self = *self % other;
    }
}

#[cfg(test)]
mod tests {
    use super::*; // use the everything above this

    #[test]
    fn new() {
        assert_eq!(Coord::new(5usize, 5), Coord{x: 5, y: 5});
        assert_eq!(Coord::new(10, 12), Coord{x: 10, y: 12});
        assert_eq!(Coord::new(12.5, 10.2), Coord{x: 12.5, y: 10.2});
        assert_eq!(Coord::new(-10, 20), Coord{x: -10, y: 20});
        assert_eq!(Coord::new(3u8, 100), Coord{x: 3, y: 100});
    }
}
