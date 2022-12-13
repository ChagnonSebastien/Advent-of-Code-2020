use std::ops::AddAssign;

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
pub(crate) struct Vector2D<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> AddAssign<Vector2D<T>> for Vector2D<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Vector2D<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}