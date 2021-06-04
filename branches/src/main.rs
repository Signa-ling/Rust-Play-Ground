fn main() {
    let condition = true;

    // Since the if statement is an expression, it can be brought to the right side of the let statement.
    // However, the types returned in if and else.
    let number = if condition{
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}