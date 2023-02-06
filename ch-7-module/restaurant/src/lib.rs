// From Parent Module to access the Child Module, Need to make the Module and Function Public
// From Child Module to access the parent Module, Can use "Super" keyword

mod front_of_house {
    pub mod hosting { // Without Pub Keyword, accessing through "front_of_house" module doesn't work
        pub fn add_to_waitlist() {}
        // fn seat_at_table() {}
    }

    /* mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    } */
}
pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
    pub struct Breakfast { // Struct to public if wanted to access from public function
        pub toast: String, // Need to make the struct field pub, if wanted to change outside.
        seasonal_fruit: String
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // Function to be Public if wanted to access from public function
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }
}
pub fn eat_at_restaurant_new() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
}
