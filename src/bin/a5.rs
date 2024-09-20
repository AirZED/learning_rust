// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut i = 1;

    loop {
        println!("{:?}", i);

        i = i + 1;

        if i == 5 {
            break;
        }
    }

    println!("END OF FIRST LOOP");

    let mut a = 0;

    while a < 6 {
        println!("{:?}", a);

        a = a + 1;
    }
}
