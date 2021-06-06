mod front_of_house;
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        // If the path starts relative to the parent module, it can be called by prefixing it with super
        super::serve_order();
    }

    fn cook_order() {}

    // If you use pub before defining the structure, the structure will be exposed.
    pub struct Breakfast {
        // Each field can be set individually to publish or not, depending on whether it has a pub or not.
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Since back_of_house::Breakfast has a private field, 
        // a public related function to create an instance of Breakfast must be provided by the structure.
        // (Named "summer" here.)
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // When you publish an enum, all variants are published.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Using the use keyword, once a path has been brought into scope,
// it can be called as if the elements in the path were local.
use crate::front_of_house::hosting;

// It can also be written using relative paths as follows
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // Since we are using the use keyword, we can call it as follows
    hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; 
    // we're not allowed to see or modify the seasonal_fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}