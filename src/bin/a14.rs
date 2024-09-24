// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}
// * Create and store at least 3 people in a vector

// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

fn print_info(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person { age: 9, name: "Solomon".to_owned(), favorite_color: "Green".to_owned() },
        Person { age: 10, name: String::from("Solomon"), favorite_color: "Yellow".to_owned() },
        Person { age: 75, name: String::from("Solomon"), favorite_color: "Black".to_owned() }
    ];

    for person in people {
        if person.age <= 10 {
            print_info(&person.name);
            print_info(&person.favorite_color);
        }
    }
}
