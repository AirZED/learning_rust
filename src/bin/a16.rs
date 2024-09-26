// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let okon = Student {
        name: "okon".to_owned(),
        locker: None,
    };

    let atim = Student {
        name: "Atim".to_owned(),
        locker: Some(54),
    };

    match atim.locker {
        Some(locker) => println!("This student's locker is {:?}", locker),
        None => println!("That block got no locker"),
    }
}
