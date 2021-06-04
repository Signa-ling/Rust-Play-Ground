fn main() {
    // loop_func();
    while_func();
    for_func();

    // Equal to while_func()
    lift_off();
}

fn loop_func() {
    // The loop keyword will continue to work forever until you stop the program manually.
    // Press Ctrl+C to stop.
    loop {
        println!("again!");
    }
}

fn while_func() {
    // The while keyword will be repeated while the conditional expression is true.
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!");
}

fn for_func() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn lift_off() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}