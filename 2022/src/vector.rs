use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, Mul, Div};
use num_traits::Signed;


#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
pub(crate) struct Vector2D<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: AddAssign<T>> AddAssign<Vector2D<T>> for Vector2D<T> {
    fn add_assign(&mut self, rhs: Vector2D<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Add<T, Output = T>> Add<Vector2D<T>> for Vector2D<T> {
    type Output = Vector2D<T>;

    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<T, Output = T>> Sub<Vector2D<T>> for Vector2D<T> {
    type Output = Vector2D<T>;

    fn sub(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Vector2D<T> {
    type Output = Vector2D<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Div<T, Output = T> + Copy> Div<T> for Vector2D<T> {
    type Output = Vector2D<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Add<T, Output = T> + Signed> Vector2D<T> {
    pub fn manhattan_length(self) -> T {
        self.x.abs() + self.y.abs()
    }
}

impl<T: Display> Display for Vector2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}
