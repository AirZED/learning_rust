// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let some_value = 50;

    match some_value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }

    let mut i = 3;

    loop {
        println!("{:?}", i);

        i = i + 1;

        if i == 5 {
            break;
        }
    }
}
