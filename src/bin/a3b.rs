// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to disptruelay messages to the terminal

fn main() {
    let my_variable = 4;

    if my_variable > 5 {
        println!(">5");
    } else if my_variable < 5 {
        println!("<5");
    } else {
        println!("=5");
    }

    // using match operation

    let some_int = 3;

    match some_int {
        1 => println!("Its 1"),
        2 => println!("Its 2"),
        3 => println!("Its 3"),
        _ => println!("Its something else"),
    }
}
