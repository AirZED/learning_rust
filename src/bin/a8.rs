// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Vanilla,
    Orange,
}

struct DrinkInfo {
    flavor: Flavour,
    fluid_ounce: f64,
}

fn print_drink(drink:DrinkInfo) {
    match drink.flavor {
        Flavour::Vanilla => println!("Vanilla"),
        Flavour::Orange => println!("Orange"),
    }
    println!("{:?}", drink.fluid_ounce)
}

fn main() {
    let _my_drink = DrinkInfo {
        flavor: Flavour::Vanilla,
        fluid_ounce: 24.87,
    };

    print_drink(_my_drink)
}
