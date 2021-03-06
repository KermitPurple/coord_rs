//! A simple coordinate library to make manipulation of points easier

use num::{Float, Num};
use rand::{distributions::Standard, prelude::*};
use std::ops::*;

pub fn distancef<T: Float + Copy>(start: impl Into<Coord<T>>, end: impl Into<Coord<T>>) -> T {
    let start = start.into();
    let end = end.into();
    ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt()
}

pub fn distance(start: impl Into<Coord<f32>>, end: impl Into<Coord<f32>>) -> f32 {
    distancef(start, end)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Coord<T: Num + Copy> {
    x: T,
    y: T,
}

impl<T: Num + Copy> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn into<J: Num + Copy + From<T>>(self) -> Coord<J> {
        Coord {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl<T: Num + Copy> Coord<T>
where
    f32: From<T>,
{
    pub fn distance(self, other: impl Into<Coord<f32>>) -> f32 {
        distance(self.into(), other.into())
    }
}

impl<T: Float + Copy> Coord<T> {
    pub fn distancef<I: Into<Coord<T>>>(self, other: I) -> T {
        distancef(self, other)
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

impl<T: Num + Copy> From<T> for Coord<T> {
    fn from(val: T) -> Self {
        Self { x: val, y: val }
    }
}

impl<T: Num + Copy> From<(T, T)> for Coord<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Copy> From<&(T, T)> for Coord<T> {
    fn from(&(x, y): &(T, T)) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Copy> From<[T; 2]> for Coord<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Copy> From<&[T; 2]> for Coord<T> {
    fn from(&[x, y]: &[T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Copy> TryFrom<Vec<T>> for Coord<T> {
    type Error = ();
    fn try_from(v: Vec<T>) -> Result<Self, ()> {
        if v.len() == 2 {
            Ok(Self { x: v[0], y: v[1] })
        } else {
            Err(())
        }
    }
}

impl<T: Num + Copy> Distribution<Coord<T>> for Standard
where
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Coord<T> {
        Coord::new(rng.gen(), rng.gen())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // use the everything above this

    #[test]
    fn new() {
        assert_eq!(Coord { x: 5, y: 5 }, Coord::new(5usize, 5));
        assert_eq!(Coord { x: 10, y: 12 }, Coord::new(10, 12));
        assert_eq!(Coord { x: 12.5, y: 10.2 }, Coord::new(12.5, 10.2));
        assert_eq!(Coord { x: -10, y: 20 }, Coord::new(-10, 20));
        assert_eq!(Coord { x: 3, y: 100 }, Coord::new(3u8, 100));
    }

    #[test]
    fn test_distance() {
        let a = Coord::new(10f32, 20.);
        let b = Coord::new(20f32, 30.);
        assert_eq!(distancef(a, b), a.distancef(b));
        let a = Coord::new(10u16, 20);
        let b = Coord::new(20u16, 30);
        assert_eq!(distance(a.into(), b.into()), a.distance(b.into()));
    }

    #[test]
    fn add() {
        assert_eq!(
            Coord::new(9, 3) + Coord::new(1, 17),
            Coord::new(10usize, 20)
        );
        assert_eq!(Coord::new(9, -3) + Coord::new(-19, 23), Coord::new(-10, 20));
        assert_eq!(
            Coord::new(10., 4.) + Coord::new(-3.5, -0.8),
            Coord::new(6.5, 3.2)
        );
        assert_eq!(Coord::new(20, 0) + Coord::new(0, 20), Coord::new(20, 20));
    }

    #[test]
    fn add_assign() {
        let mut coord = Coord::new(32, 10);
        coord += Coord::new(-12, 10);
        assert_eq!(coord, Coord::new(20, 20));
        coord += Coord::new(5, 2);
        assert_eq!(coord, Coord::new(25, 22));
        coord += Coord::new(-50, -50);
        assert_eq!(coord, Coord::new(-25, -28));
        let mut coord = Coord::new(10usize, 3);
        coord += Coord::new(1, 3);
        assert_eq!(coord, Coord::new(11, 6));
        let mut coord = Coord::new(10., 3.);
        coord += Coord::new(0.5, 0.1);
        assert_eq!(coord, Coord::new(10.5, 3.1));
    }

    #[test]
    fn sub() {
        assert_eq!(Coord::new(9, 17) - Coord::new(1, 17), Coord::new(8usize, 0));
        assert_eq!(Coord::new(9, -3) + Coord::new(-19, 23), Coord::new(-10, 20));
        assert_eq!(
            Coord::new(10., 4.) + Coord::new(-3.5, -0.8),
            Coord::new(6.5, 3.2)
        );
        assert_eq!(Coord::new(20, 0) + Coord::new(0, 20), Coord::new(20, 20));
    }

    #[test]
    fn sub_assign() {
        let mut coord = Coord::new(32, 10);
        coord -= Coord::new(10, 20);
        assert_eq!(coord, Coord::new(22, -10));
        let mut coord = Coord::new(10usize, 5usize);
        coord -= Coord::new(9, 5);
        assert_eq!(coord, Coord::new(1, 0));
    }

    #[test]
    fn mul() {
        assert_eq!(Coord::new(5usize, 3) * Coord::new(2, 3), Coord::new(10, 9));
        assert_eq!(Coord::new(10, -3) * Coord::new(-2, -3), Coord::new(-20, 9));
    }

    #[test]
    fn mul_assign() {
        let mut coord = Coord::new(2usize, 10);
        coord *= Coord::new(5, 6);
        assert_eq!(coord, Coord::new(10, 60));
        let mut coord = Coord::new(-5, 32);
        coord *= Coord::new(-5, 1);
        assert_eq!(coord, Coord::new(25, 32));
    }

    #[test]
    fn div() {
        assert_eq!(Coord::new(5usize, 3) / Coord::new(2, 3), Coord::new(2, 1));
        assert_eq!(Coord::new(10, -3) / Coord::new(-2, -3), Coord::new(-5, 1));
    }

    #[test]
    fn div_assign() {
        let mut coord = Coord::new(2usize, 10);
        coord /= Coord::new(5, 6);
        assert_eq!(coord, Coord::new(0, 1));
        let mut coord = Coord::new(-5, 32);
        coord /= Coord::new(-5, 1);
        assert_eq!(coord, Coord::new(1, 32));
    }

    #[test]
    fn rem() {
        assert_eq!(Coord::new(5usize, 3) % Coord::new(2, 3), Coord::new(1, 0));
        assert_eq!(Coord::new(10, -3) % Coord::new(-2, -3), Coord::new(0, 0));
    }

    #[test]
    fn rem_assign() {
        let mut coord = Coord::new(2usize, 10);
        coord %= Coord::new(5, 6);
        assert_eq!(coord, Coord::new(2, 4));
        let mut coord = Coord::new(-5, 32);
        coord %= Coord::new(-5, 1);
        assert_eq!(coord, Coord::new(0, 0));
    }

    #[test]
    fn from() {
        // numbers
        assert_eq!(Coord::new(0u8, 0), 0.into());
        assert_eq!(Coord::new(0u16, 0), 0.into());
        assert_eq!(Coord::new(0u32, 0), 0.into());
        assert_eq!(Coord::new(0u64, 0), 0.into());
        assert_eq!(Coord::new(0usize, 0), 0.into());
        assert_eq!(Coord::new(0i8, 0), 0.into());
        assert_eq!(Coord::new(0i16, 0), 0.into());
        assert_eq!(Coord::new(0i32, 0), 0.into());
        assert_eq!(Coord::new(0i64, 0), 0.into());
        assert_eq!(Coord::new(0isize, 0), 0.into());
        assert_eq!(Coord::new(0.0f32, 0.0), 0.0.into());
        assert_eq!(Coord::new(0.0f64, 0.0), 0.0.into());
        // objects
        assert_eq!(Coord::new(0, 1), [0, 1].into());
        assert_eq!(Coord::new(0, 1), (0, 1).into());
        assert_eq!(Coord::new(0, 1), vec![0, 1].try_into().unwrap());
        assert_eq!(Coord::new(0f32, 1f32), Coord::new(0i16, 1i16).into());
        // operations
        let mut coord = Coord::new(0, 1);
        coord += (10, 9);
        assert_eq!(coord, (10, 10).into());
        coord -= 5;
        assert_eq!(coord, (5, 5).into());
        coord *= 10;
        assert_eq!(coord, (50, 50).into());
        coord /= 2;
        assert_eq!(coord, (25, 25).into());
        coord *= (-1, 2);
        assert_eq!(coord, (-25, 50).into());
    }
}
