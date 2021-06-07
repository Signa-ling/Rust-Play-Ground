// Cannot use ">" unless PatrialOrd is specified as a trait boundary.
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() { 
        if item > largest {
            largest = item;
        }
    }

    &largest
}

// If you don't provide a generic type, you will need to describe the same content for each type.
/*
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// If a field has more than one possible type, multiple generic type arguments need to be used.
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

// Need to declare a generic type after impl.
impl<T, U> Point<T, U> {
    // The generic type argument of a structure definition is not necessarily the same as the one used in the method signature of that structure.
    // However, V and W, which appear below, are only relevant to the mixup method, so they are declared after this method.
    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
