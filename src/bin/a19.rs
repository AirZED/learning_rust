// Topic: HashMap
use std::collections::HashMap;
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

fn main() {
    let mut items = HashMap::new();

    items.insert("Chairs", 5);
    items.insert("Beds", 3);
    items.insert("Tables", 2);
    items.insert("Couches", 0);

    let mut total = 0;

    for (item, amount) in items.iter() {
        total += amount;
    }

    if total == 0 {
        return println!("Out of stock");
    }
    println!("{:?}", total)
}
