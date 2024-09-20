// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Yellow,
    Purple,
    Green,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        Color::Purple => println!("Purple"),
        Color::Green => println!("Green"),
    }
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };
    println!("Stock: {:?}", cereal.stock);

    print_color(Color::Red);
}
