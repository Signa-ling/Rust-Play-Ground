// Conditional separation of method implementations using trait boundaries.
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> { 
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

use std::fmt::Display;
impl<T: Display +  PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }

}