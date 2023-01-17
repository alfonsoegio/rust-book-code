use packcrates::eat_at_restaurant;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, world!");
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
    eat_at_restaurant()
}
