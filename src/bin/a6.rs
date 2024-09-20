// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut a = 5;

    while a > 0 {
        println!("{:?}", a);
        a = a - 1;
    }

    println!("Done!");

    // ENUMS

    fn which_way(go: Direction) -> String {
        match go {
            Direction::Up => "hmmp".to_string(),
            Direction::Down => "down".to_string(),
            Direction::Left => "left".to_string(),
            Direction::Right => "right".to_string(),
        }
    }

    let way = which_way(Direction::Up);

    println!("{:?}", way)
}
