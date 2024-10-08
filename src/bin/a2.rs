// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result() {
    let _sum = add(2, 6);

    println!("Sum is {:?}", _sum);
}

fn main() {

    display_result();

    let age = 15;
    if age >= 21 {
        println!("You can drink");
    } else {
        println!("You can't drink");
    }
}
