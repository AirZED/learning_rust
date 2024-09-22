// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    fn get_coord() -> (i32, i32) {
        (2, 5)
    }

    let (x, y) = get_coord();

    if y > 5 {
        println!("Greater than 5");
    } else if y < 5 {
        println!("Less then 5");
    } else {
        println!("Equals to 5");
    }

    let access_level = Access::Admin;

    // this restrict access to only admins
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("Can access: {:?}", can_access_file);
}
