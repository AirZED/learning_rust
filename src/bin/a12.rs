// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    BLACK,
    GRAY,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::BLACK => println!("Black"),
            BoxColor::GRAY => println!("Grey"),
        }
    }
}

struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth)
    }
}

struct ShoppingBox {
    dimensions: Dimensions,
    weight: f32,
    color: BoxColor,
}

impl ShoppingBox {
    fn new(weight: f32, dimensions: Dimensions, color: BoxColor) -> Self {
        Self {
            weight,
            dimensions,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("The weight of the box is {:?}", self.weight);
    }
}

fn main() {
    let dims = Dimensions {
        height: 65.7,
        width: 99.09,
        depth: 40.0,
    };
    let my_box = ShoppingBox::new(23.87, dims, BoxColor::GRAY);

    my_box.print()
}
