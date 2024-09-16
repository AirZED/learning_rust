// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn display_firstname() {
    println!("Mfoniso");
}

fn display_lastname() {
    println!("Ukpabio");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    display_firstname();
    display_lastname();
    let _sum = add(2, 6);

    println!("Sum is {:?}", _sum);

    let addVal = 2 + 2;
    let subVal = 2 - 2;
    let mulVal = 2 * 2;
    let divVal = 2 / 2;
    let remVal = 2 % 2;
}
