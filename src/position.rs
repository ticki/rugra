/// A trait for types describing a position
pub trait Position<T> {
    /// Get the x coordinate
    fn x(&self) -> T;
    /// Get the y coordinate
    fn y(&self) -> T;
}

impl<T: Copy> Position<T> for (T, T) {
    fn x(&self) -> T {
        self.0
    }

    fn y(&self) -> T {
        self.1
    }
}
