use std::io; // The io library supports input-output
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                .expect("Failed to read line");  // To make it easier to read, I made one line for each method.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}