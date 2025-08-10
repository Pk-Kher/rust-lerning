use crate::Asparagus;
mod vegetables;

// #[derive(Debug)]
// pub struct Garden1 {
//     pub name: String,
// }
pub fn show_garden_aspa() {
    let plant=Asparagus{};
    println!("inside garden: {plant:?}");
}
