// Even debugging your own implementation of a type can be done easily.
#[derive(Debug)]

// An enum is a type in which one of several values is chosen. 
// For example, it is used when you want to create a type such as "variable c has a value of red, green, or blue.
enum UsState {
    Alabama,
    Alaska,
    // ...etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // The enumerator can hold additional information.
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    // The result of the expression of the matched arm becomes the return value of the entire match expression.
    match coin {
        Coin::Penny => {
            // Use curly brackets when you want to run multiple lines of code in the match arm.
            // Return 1 after the println! statement is output in the following cases.
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    // Enumerators can be instantiated as follows.
    let penny_coin = Coin::Penny;
    let quarter_coin_alabama = Coin::Quarter(UsState::Alabama);
    let quarter_coin_alaska = Coin::Quarter(UsState::Alaska);

    println!("Penny coin is {} cents!", value_in_cents(penny_coin));
    println!("Quarter coin is {} cents!", value_in_cents(quarter_coin_alabama));
    println!("Quarter coin is {} cents!", value_in_cents(quarter_coin_alaska));
}
