// If the pub is not attached, it cannot be accessed by eat_at_resutaurant().
// If you write the file name (front_of_house in this case), you can also call its content (hosting).
pub mod hosting {
    // Same here.
    pub fn add_to_waitlist() {}
}

// This has the same meaning as defining the following text in lib.rs
// mod front_of_house {
//     pub mod hosting {
//        pub fn add_to_waitlist() {}
//    }
// }

// If you write the following sentence in src/front_of_house/hosting.rs, 
// it still works because the module tree is the same.
// pub fn add_to_waitlist() {}