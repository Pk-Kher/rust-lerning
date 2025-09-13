pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    pub fn new(width: i32, height: i32) -> Rectangle {
        Rectangle { width, height }
    }
}
