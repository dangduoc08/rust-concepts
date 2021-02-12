#[derive(Debug)]
pub struct Point<T, U, D> {
    pub x: T,
    pub y: U,
    pub z: D,
}

#[derive(Debug)]
pub enum Status<T> {
    Success(T),
    Error(T),
    Cancel(T),
}

impl<T, U, D> Point<T, U, D> {
    pub fn draw(&self) -> (&T, &U, &D) {
        (&self.x, &self.y, &self.z)
    }
}
