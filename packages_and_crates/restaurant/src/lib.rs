use crate::garden::garden::GardenDetails;

pub mod garden;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        // fn seat_at_table(){}
    }
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
mod back_of_house {
    // use crate::deliver_order;
    // or "super"
    use super::*;
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast: toast,
                seasonal_fruit: String::from("mango"),
            }
        }
    }
    //if enum is public then all element of enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        // let gard = gardenMod::Garden {};
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}
pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //
    front_of_house::hosting::add_to_waitlist();
    // let mut meal = back_of_house::Breakfast::summer(String::from("bread"));
    // meal.toast=String::from("any bread");
    // println!("i'd like {} toast please",meal.toast);
    let order1 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

fn my_garden() {
    let g1 = GardenDetails::new(String::from("nothing"), String::from("something"));
    println!("{:#?}", g1);
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        my_garden();
    }
}
