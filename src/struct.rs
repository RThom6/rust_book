#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement methods for Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Returns whether Rectangle can hold another rectangle based on their sizes
    fn can_hold(&self, &other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}