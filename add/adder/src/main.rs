extern crate add_one;
extern crate add_two;

fn main() {
    let num = 10;
    let add_one_num = add_one::add_one(num);
    println!("Hello, world! {} plus one is {}!", num, add_one_num);
    println!("{} plus two is {}!", add_one_num, add_two::add_two(add_one_num));
}
