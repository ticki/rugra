pub trait Position<T> {
    fn x(&self) -> T;
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
