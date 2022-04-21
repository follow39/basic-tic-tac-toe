#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    pub fn x(self) -> usize {
        self.x
    }
    pub fn y(self) -> usize {
        self.y
    }
}
