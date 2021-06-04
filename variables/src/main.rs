fn main() {
    check_mut_operation();
    shadowing();
    let x = plus_one(5);
    println!("five plus one = {}", x);
}

fn check_mut_operation() {
    let mut x = 5;
    println!("The value of mut x is: {}", x);
    x = 6;  // The value of a variable can be rewritten by using mut.
    println!("The value of mut x is: {}", x);
}

fn shadowing() {
    let x = 5;  // The value of an immutable variable can be rewritten without using mut.
    let x = x + 1;
    let x = x * 2;
    println!("The value of shadowing x is {}", x);
}

fn plus_one(x: i32) -> i32 {
    // It's an expression, so no semicolon.
    // If you add a semicolon here, it will be treated as a sentence.
    // (That would be an error.)
    x + 1
}